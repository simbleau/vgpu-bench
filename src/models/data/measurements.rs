use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use csv::Writer;
use log::warn;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::util;
use crate::Measurable;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Measurements {
    measurables: HashMap<String, Vec<Measurable>>,
}

impl Measurements {
    pub fn new() -> Self {
        Measurements {
            measurables: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.measurables.len()
    }

    pub fn insert<S: Into<String>>(&mut self, key: S, measurable: Measurable) {
        let key: String = key.into();
        if let None = self.measurables.get(&key) {
            self.measurables.insert(key.clone(), vec![]);
        }
        let measurables = self.measurables.get_mut(&key).unwrap();
        measurables.push(measurable);
    }

    pub fn clear(&mut self) {
        self.measurables.clear();
    }

    pub fn columns(&self) -> Vec<&String> {
        self.measurables.keys().collect()
    }

    pub fn rows(&self) -> Vec<&Vec<Measurable>> {
        self.measurables.values().collect()
    }

    pub fn col_count(&self) -> usize {
        self.measurables.keys().len()
    }

    pub fn row_count(&self) -> usize {
        // All rows should be the same length
        match self.measurables.values().next() {
            Some(item) => item.len(),
            None => 0,
        }
    }

    pub fn write_csv<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        // Assert all rows are the same length
        let mut min_rows = 0;
        for (key, measurables) in self.measurables.iter() {
            if measurables.len() > min_rows {
                if min_rows != 0 {
                    warn!("{key} has more rows than other elements");
                }
                min_rows = measurables.len();
            }
        }

        let mut file = util::io::create_or_append(&path)?;
        // Write header
        if file.metadata().unwrap().len() == 0 {
            let header: String = self
                .measurables
                .keys()
                .cloned()
                .intersperse(",".to_string())
                .collect();
            writeln!(file, "{header}")?;
        }
        // Write rows
        let mut writer = util::io::csv_writer(path)?;
        for row in 0..min_rows {
            let mut row_items = vec![];
            for col_measurables in self.measurables.values() {
                let row_item = col_measurables.get(row);
                row_items.push(row_item.unwrap()); // Safety: rows checked above
            }
            writer.serialize(row_items)?;
        }
        writer.flush()?;

        Ok(())
    }
}

impl Measurements {
    // TODO remove need for this
    pub fn to_pystring<'py>(&self, py: Python<'py>) -> PyResult<&'py PyString> {
        // Write rows
        let mut writer = Writer::from_writer(vec![]);
        for row in 0..self.row_count() {
            let mut row_items = vec![];
            for measurables in self.measurables.values() {
                let row_item = measurables.get(row);
                row_items
                    .push(row_item.expect("Are all rows the same length?"));
            }
            writer.serialize(row_items).unwrap();
        }
        writer.flush()?;

        let data = String::from_utf8(writer.into_inner().unwrap())?;
        Ok(PyString::new(py, &data))
    }
}

impl ToPyObject for Measurements {
    fn to_object(&self, py: Python) -> PyObject {
        let mut wtr = Writer::from_writer(vec![]);
        for item in &self.measurables {
            wtr.serialize(item).unwrap();
        }
        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        PyObject::from(PyString::new(py, &data))
    }
}

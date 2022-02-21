use std::path::Path;

use csv::Writer;
use erased_serde::Serialize;
use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::AsPyPointer;

use crate::util;
use crate::Measurable;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Measurements {
    measurables: Vec<Measurable>,
}

impl Measurements {
    pub fn new() -> Self {
        Measurements {
            measurables: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.measurables.len()
    }

    pub fn push(&mut self, measurable: Measurable) {
        self.measurables.push(measurable);
    }

    pub fn clear(&mut self) {
        self.measurables.clear();
    }

    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let rows: Vec<Box<dyn Serialize>> = self
            .measurables
            .iter()
            .map(|x| -> Box<dyn Serialize> { Box::new(x.clone()) })
            .collect();
        Ok(util::io::write_csv(path, &rows)?)
    }
}

impl Measurements {
    // TODO remove need for this
    pub fn to_pystring<'py>(&self, py: Python<'py>) -> &'py PyString {
        let mut wtr = Writer::from_writer(vec![]);
        for item in &self.measurables {
            wtr.serialize(item).unwrap();
        }
        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        PyString::new(py, &data)
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

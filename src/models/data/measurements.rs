use std::path::Path;

use log::trace;
use log::warn;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::log_assert;
use crate::util;
use crate::Measurable;
use crate::Result;

#[derive(Debug)]
pub struct Measurements<T>
where
    T: Measurable,
{
    measurables: Vec<T>,
}

impl<T> Measurements<T>
where
    T: Measurable,
{
    pub fn new() -> Self {
        Measurements {
            measurables: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.measurables.len()
    }

    pub fn push(&mut self, measurement: T) {
        self.measurables.push(measurement);
    }

    pub fn clear(&mut self) {
        self.measurables.clear()
    }

    pub fn write<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let mut path = path.as_ref().to_owned();
        path.set_extension("csv");

        trace!("writing measurements to {path:?}");

        // Overwrite file if it exists
        if path.exists() {
            std::fs::remove_file(&path)?;
            log_assert!(
                path.exists() == false,
                "{path:?} could not be removed"
            );
        }

        if self.measurables.is_empty() {
            warn!("{path:?} no measurable to write, skipping");
        } else {
            let mut writer = util::io::csv_writer(path)?;
            for row in &self.measurables {
                writer.serialize(row)?;
            }
            writer.flush()?;
        }
        Ok(())
    }
}

impl<T> Measurements<T>
where
    T: Measurable,
{
    pub fn to_pystring<'py>(&self, py: Python<'py>) -> &'py PyString {
        let mut wtr = util::io::csv_string_writer();
        for item in &self.measurables {
            wtr.serialize(item).unwrap();
        }
        let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        PyString::new(py, &data)
    }
}
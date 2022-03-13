use std::path::Path;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::util;
use crate::Measurable;
use crate::Result;

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

    pub fn write<P>(&self, _path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        todo!()
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

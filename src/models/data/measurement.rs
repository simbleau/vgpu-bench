use pyo3::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Measurable {
    Integer(i64),
    Float(f64),
    Bool(bool),
}

impl ToPyObject for Measurable {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Measurable::Integer(i) => i.into_py(py),
            Measurable::Float(f) => f.into_py(py),
            Measurable::Bool(b) => b.into_py(py),
        }
    }
}

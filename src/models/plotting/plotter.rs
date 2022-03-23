use pyo3::{prelude::*, types::PyTuple};
use std::path::Path;

use crate::models::{Measurable, Measurements};
use crate::Result;

pub trait Plotter {
    fn plot<T>(&self, data: &Measurements<T>) -> Result<PyObject>
    where
        T: Measurable;

    fn save_plot<P, T>(&self, data: &Measurements<T>, path: P) -> Result<()>
    where
        P: AsRef<Path>,
        T: Measurable,
    {
        let fig = self.plot(data)?;
        let path = path.as_ref().as_os_str();

        Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::models::plot_utils(py)?;

            // Save
            let save_func: PyObject = utils.getattr("save")?.into();
            save_func.call1(py, (fig, path))?;

            Ok(0_i32.into_py(py))
        })?;

        Ok(())
    }

    fn show_plot<T>(&self, data: &Measurements<T>) -> Result<()>
    where
        T: Measurable,
    {
        let fig = self.plot(data)?;

        Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::models::plot_utils(py)?;

            // Show
            let show_func: PyObject = utils.getattr("show")?.into();
            show_func.call1(py, PyTuple::new(py, &[fig]))?;

            Ok(0_i32.into_py(py))
        })?;

        Ok(())
    }
}

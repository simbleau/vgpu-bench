use pyo3::{prelude::*, types::PyTuple};

use crate::{Measurements, Result};
use std::path::Path;

pub trait Plotter {
    fn plot(&self, data: &Measurements) -> Result<PyObject>;

    fn save_plot<P>(&self, data: &Measurements, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let fig = self.plot(data)?;
        let path = path.as_ref().as_os_str();

        Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::plot_utils(py)?;

            // Save
            let save_func: PyObject = utils.getattr("save")?.into();
            save_func.call1(py, (fig, path))?;

            Ok(0_i32.into_py(py))
        })?;

        Ok(())
    }

    fn show_plot(&self, data: &Measurements) -> Result<()> {
        let fig = self.plot(data)?;

        Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::plot_utils(py)?;

            // Show
            let show_func: PyObject = utils.getattr("show")?.into();
            show_func.call1(py, PyTuple::new(py, &[fig]))?;

            Ok(0_i32.into_py(py))
        })?;

        Ok(())
    }
}

use log::debug;
use pyo3::prelude::*;
use pyo3::types::*;

use crate::Measurements;
use crate::{Plotter, Result};

pub enum NumericPlotType {
    Line,
    Bar,
}

pub struct NumericPlotter {
    pub plot_type: NumericPlotType,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub sort_by: Option<String>,
    pub sort_ascending: bool,
    pub show_stats: bool,
    pub show_stats_table: bool,
}

impl Plotter for NumericPlotter {
    fn plot(&self, data: &Measurements) -> Result<PyObject> {
        let script = match self.plot_type {
            NumericPlotType::Line => include_str!("py/numeric_line.py"),
            NumericPlotType::Bar => unimplemented!(),
        };

        Ok(Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::plot_utils(py)?;

            // Make dataframe
            let df_func: PyObject = utils.getattr("dataframe")?.into();
            let py_data_columns = PyList::new(py, &["value"]);
            let py_data = data.to_pystring(py);
            let py_kwargs = vec![
                ("columns", py_data_columns.into_py(py)),
                ("rows", py_data.into_py(py)),
                ("sort", self.sort_by.is_some().into_py(py)),
                (
                    "by",
                    self.sort_by.as_ref().map_or(py.None(), |v| v.into_py(py)),
                ),
                ("ascending", self.sort_ascending.into_py(py)),
            ]
            .into_py_dict(py);
            let df = df_func.call(py, (), Some(py_kwargs))?;
            debug!("Numeric plotter dataframe:\n{df}");

            // Plot
            let plotter = PyModule::from_code(py, script, "", "")?;
            let plot_func: PyObject = plotter.getattr("plot")?.into();
            let plot: PyObject = plot_func
                .call1(py, (df, &self.title, &self.x_label, &self.y_label))?;

            Ok(plot)
        })?)
    }
}

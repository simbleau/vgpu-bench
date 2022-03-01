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
    pub x_column: String,
    pub y_column: String,
    pub x_label: String,
    pub y_label: String,
    pub plot_by: Option<String>,
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
            let py_data_columns = PyList::new(py, data.columns());
            let py_data = data.to_pystring(py)?;
            let py_args = (py_data_columns, py_data);
            let py_kwargs = vec![
                ("sort", self.sort_by.is_some().into_py(py)),
                (
                    "sort_by",
                    self.sort_by.as_ref().map_or(py.None(), |v| v.into_py(py)),
                ),
                ("ascending", self.sort_ascending.into_py(py)),
            ]
            .into_py_dict(py);
            let df = df_func.call(py, py_args, Some(py_kwargs))?;
            debug!("Numeric plotter dataframe:\n{df}");

            // Plot
            let plotter = PyModule::from_code(py, script, "", "")?;
            let plot_func: PyObject = plotter.getattr("plot")?.into();
            let py_args = (
                df,
                &self.x_column,
                &self.y_column,
                &self.title,
                &self.x_label,
                &self.y_label,
            );
            let py_kwargs = vec![
                (
                    "plot_by",
                    self.plot_by
                        .as_ref()
                        .map_or(py.None(), |v| v.into_py(py))
                        .into_py(py),
                ),
                ("show_stats", self.show_stats.into_py(py)),
                ("show_stats_table", self.show_stats_table.into_py(py)),
            ]
            .into_py_dict(py);

            let plot: PyObject =
                plot_func.call(py, py_args, Some(py_kwargs))?;

            Ok(plot)
        })?)
    }
}

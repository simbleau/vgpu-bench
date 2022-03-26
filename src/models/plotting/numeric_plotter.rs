use log::debug;
use pyo3::prelude::*;
use pyo3::types::*;

use crate::models::{Measurable, Measurements, Plotter};
use crate::Result;

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
    fn plot<T>(&self, data: &Measurements<T>) -> Result<PyObject>
    where
        T: Measurable,
    {
        let script = match self.plot_type {
            NumericPlotType::Line => include_str!("py/numeric_line.py"),
            NumericPlotType::Bar => unimplemented!(),
        };

        Ok(Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::models::plot_utils(py)?;

            // Make dataframe
            let df_func: PyObject = utils.getattr("dataframe")?.into();
            let py_data_columns =
                PyList::new(py, &["filename", "frame", "time_ns"]);
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

            let x_col = String::from("filename");
            let y_col = String::from("frametime_ns");
            let py_kwargs = vec![
                ("plot_by", "".into_py(py)),
                ("show_stats", true.into_py(py)),
                ("show_stats_table", true.into_py(py)),
            ]
            .into_py_dict(py);
            let plot: PyObject = plot_func.call(
                py,
                (
                    df,
                    &x_col,
                    &y_col,
                    &self.title,
                    &self.x_label,
                    &self.y_label,
                ),
                Some(py_kwargs),
            )?;

            Ok(plot)
        })?)
    }
}

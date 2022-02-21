use log::debug;
use pyo3::prelude::*;
use pyo3::types::*;

use crate::Measurements;
use crate::{Plotter, Result};

pub enum BooleanPlotType {
    Pie,
    Stepper,
}

pub struct BooleanPlotter {
    pub plot_type: BooleanPlotType,
    pub title: String,
    pub true_label: String,
    pub false_label: String,
}

impl Plotter for BooleanPlotter {
    fn plot(&self, data: &Measurements) -> Result<PyObject> {
        let script = match self.plot_type {
            BooleanPlotType::Pie => include_str!("py/boolean.py"),
            BooleanPlotType::Stepper => unimplemented!(),
        };

        Ok(Python::with_gil(|py| -> PyResult<Py<PyAny>> {
            // Load util functions
            let utils = crate::plot_utils(py)?;

            // Make dataframe
            let df_func: PyObject = utils.getattr("dataframe")?.into();
            let py_data_columns = PyList::new(py, &["value"]);
            let py_data = data.to_pystring(py);
            let df = df_func.call1(py, (py_data_columns, py_data))?;
            debug!("Boolean plotter dataframe:\n{df}");

            // Plot
            let plotter = PyModule::from_code(py, script, "", "")?;
            let plot_func: PyObject = plotter.getattr("plot")?.into();
            let plot: PyObject = plot_func.call1(
                py,
                (df, &self.title, &self.true_label, &self.false_label),
            )?;

            Ok(plot)
        })?)
    }
}

#[cfg(test)]
#[test]
fn test_run() {
    use crate::Measurable;

    let mut data = Measurements::new();
    data.push(Measurable::Bool(true));
    data.push(Measurable::Bool(false));
    data.push(Measurable::Bool(false));
    let plotter = BooleanPlotter {
        plot_type: BooleanPlotType::Pie,
        title: "My title".to_string(),
        true_label: "The truths".to_string(),
        false_label: "Falz".to_string(),
    };
    plotter.show_plot(&data).unwrap();
}

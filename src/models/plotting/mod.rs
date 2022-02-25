use pyo3::prelude::*;
pub fn plot_utils<'p>(py: Python<'p>) -> PyResult<&'p PyModule> {
    Ok(PyModule::from_code(
        py,
        include_str!("py/util.py"),
        "util",
        "util",
    )?)
}

mod plotter;
pub use plotter::Plotter;

mod boolean_plotter;
pub use boolean_plotter::BooleanPlotType;
pub use boolean_plotter::BooleanPlotter;

mod numeric_plotter;
pub use numeric_plotter::NumericPlotType;
pub use numeric_plotter::NumericPlotter;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

pub fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        // Libraries
        let sys = ("sys", py.import("sys")?);
        let os = ("os", py.import("os")?);
        // 3rd Party Libraries
        let plt = ("plt", py.import("matplotlib")?);
        let pd = ("pd", py.import("pandas")?);
        let np = ("np", py.import("numpy")?);
        // Used libraries
        let locals = [os, plt, pd, np].into_py_dict(py);

        let python_version: String = sys.1.getattr("version")?.extract()?;
        let plt_version: String = plt.1.getattr("__version__")?.extract()?;
        let pd_version: String = pd.1.getattr("__version__")?.extract()?;
        let np_version: String = np.1.getattr("__version__")?.extract()?;

        println!("Python: {python_version}\n\tmatplotlib: {plt_version}\n\tpandas: {pd_version}\n\tNumpy: {np_version}");

        let code = r#"
plt.__version__"#;
        let stdout: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("{}", stdout);
        Ok(())
    })
}

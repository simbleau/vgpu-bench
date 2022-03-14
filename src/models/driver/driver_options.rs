use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DriverOptions {
    pub(crate) output_dir: PathBuf,
    pub(crate) benchmarks_dir: PathBuf,
}

impl Default for DriverOptions {
    fn default() -> Self {
        DriverOptions {
            output_dir: PathBuf::from("output"),
            benchmarks_dir: PathBuf::from("output/benchmarks"),
        }
    }
}

impl DriverOptions {
    pub fn new(output_dir_name: &str, benchmarks_dir_name: &str) -> Self {
        let output_dir = PathBuf::from(output_dir_name);
        let benchmarks_dir = output_dir.join(benchmarks_dir_name);
        DriverOptions {
            output_dir,
            benchmarks_dir,
        }
    }
}

impl DriverOptions {
    pub fn output_dir(&self) -> &Path {
        &self.output_dir.as_path()
    }
    pub fn benchmarks_dir(&self) -> &Path {
        &self.benchmarks_dir.as_path()
    }
}

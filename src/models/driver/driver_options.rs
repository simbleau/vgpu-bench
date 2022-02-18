use std::path::Path;

pub struct DriverOptions<'a> {
    pub(crate) output_dir: &'a Path,
    pub(crate) benchmarks_dir: &'a Path,
}

impl Default for DriverOptions<'_> {
    fn default() -> Self {
        DriverOptions {
            output_dir: Path::new("output"),
            benchmarks_dir: Path::new("output/benchmarks"),
        }
    }
}

impl DriverOptions<'_> {
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
    pub fn benchmarks_dir(&self) -> &Path {
        &self.benchmarks_dir
    }
}

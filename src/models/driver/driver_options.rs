use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum DriverWriteMode {
    /// Panic if the output landing exists already, good for requiring
    /// manually user intervention to prepare the landing.
    NoMash,
    /// Purge the output landing to prepare for new output, ensuring no
    /// unrelated output exists.
    Purge,
    /// Allow data to exist in the output landing prior to start, and overwrite
    /// as necessary.
    Relaxed,
}

impl Default for DriverWriteMode {
    fn default() -> Self {
        DriverWriteMode::Relaxed
    }
}

#[derive(Debug, Clone)]
pub struct DriverOptions {
    pub(crate) output_dir: PathBuf,
    pub(crate) benchmarks_dir: PathBuf,
    pub(crate) write_mode: DriverWriteMode,
}

impl Default for DriverOptions {
    fn default() -> Self {
        DriverOptions::new("output", "benchmarks", DriverWriteMode::default())
    }
}

impl DriverOptions {
    pub(crate) fn new(
        output_dir_name: &str,
        benchmarks_dir_name: &str,
        write_mode: DriverWriteMode,
    ) -> Self {
        let output_dir = PathBuf::from(output_dir_name);
        let benchmarks_dir = output_dir.join(benchmarks_dir_name);
        DriverOptions {
            output_dir,
            benchmarks_dir,
            write_mode,
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
    pub fn write_mode(&self) -> &DriverWriteMode {
        &self.write_mode
    }
}

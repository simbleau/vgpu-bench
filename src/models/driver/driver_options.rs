use std::path::{Path, PathBuf};

/// State-Machine Definitions for handling output buffer logic.
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

    /// Generates a Default State of Relaxed.
    fn default() -> Self {
        DriverWriteMode::Relaxed
    }
}

/// Metadata for handling and storing of output for Benchmarks.
#[derive(Debug, Clone)]
pub struct DriverOptions {
    pub(crate) output_dir: PathBuf,
    pub(crate) write_mode: DriverWriteMode,
    pub(crate) on_error_continue: bool,
}

impl Default for DriverOptions {

    /// Generates a DriverOptions type with:
    /// + output_dir_name: 'output'
    /// + write_mode: DriverWriteMode::Relaxed
    /// + on_error_continue: false
    fn default() -> Self {
        DriverOptions::new("output", DriverWriteMode::default(), false)
    }
}

impl DriverOptions {
    pub(crate) fn new(
        output_dir_name: &str,
        write_mode: DriverWriteMode,
        on_error_continue: bool,
    ) -> Self {
        let output_dir = PathBuf::from(output_dir_name);
        DriverOptions {
            output_dir,
            write_mode,
            on_error_continue,
        }
    }
}

impl DriverOptions {
    pub fn output_dir(&self) -> &Path {
        &self.output_dir.as_path()
    }
    pub fn write_mode(&self) -> &DriverWriteMode {
        &self.write_mode
    }
    pub fn on_error_continue(&self) -> &bool {
        &self.on_error_continue
    }
}

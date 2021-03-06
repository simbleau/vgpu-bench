use std::path::Path;

use crate::models::{
    Benchmark, Driver, DriverOptions, DriverWriteMode, Measurable,
};

// Driver builder
pub struct DriverBuilder<T>
where
    T: Measurable,
{
    pub(crate) options: DriverOptions,
    pub(crate) benchmarks: Vec<Benchmark<T>>,
}

impl<T> DriverBuilder<T>
where
    T: Measurable,
{
    pub fn new() -> Self {
        Self {
            options: DriverOptions::default(),
            benchmarks: Vec::new(),
        }
    }

    pub fn on_error_contune(mut self, should_continue: bool) -> Self {
        self.options.on_error_continue = should_continue;
        self
    }

    /// Toggle Driver Options type's write mode for results.
    pub fn write_mode(mut self, write_mode: DriverWriteMode) -> Self {
        self.options.write_mode = write_mode;
        self
    }

    /// Assign output directory to store the result of Benchmarks.
    pub fn output_dir(mut self, output_dir: &Path) -> Self {
        self.options.output_dir = output_dir.to_owned();
        self
    }

    /// Builder Pattern wrapper for adding new Benchmarks.
    pub fn add(mut self, benchmark: Benchmark<T>) -> Self {
        self.benchmarks.push(benchmark);
        self
    }

    /// Create new Driver type instance with Builder defined DriverOptions and Benchmarks.
    pub fn build(self) -> Driver<T> {
        Driver {
            options: self.options,
            benchmarks: self.benchmarks,
        }
    }
}

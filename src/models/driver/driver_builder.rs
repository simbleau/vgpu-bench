use simplelog::SharedLogger;
use std::path::Path;

use crate::{Benchmark, Driver, DriverOptions, DriverWriteMode, Measurable};

// Driver builder
pub struct DriverBuilder<T>
where
    T: Measurable,
{
    pub(crate) options: DriverOptions,
    pub(crate) loggers: Vec<Box<dyn SharedLogger>>,
    pub(crate) benchmarks: Vec<Benchmark<T>>,
}

impl<T> DriverBuilder<T>
where
    T: Measurable,
{
    pub fn new() -> Self {
        Self {
            options: DriverOptions::default(),
            loggers: Vec::new(),
            benchmarks: Vec::new(),
        }
    }

    pub fn on_error_contune(mut self, should_continue: bool) -> Self {
        self.options.on_error_continue = should_continue;
        self
    }

    pub fn write_mode(mut self, write_mode: DriverWriteMode) -> Self {
        self.options.write_mode = write_mode;
        self
    }

    pub fn output_dir(mut self, output_dir: &Path) -> Self {
        self.options.output_dir = output_dir.to_owned();
        self
    }

    pub fn logger(mut self, logger: Box<dyn SharedLogger>) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn add(mut self, benchmark: Benchmark<T>) -> Self {
        self.benchmarks.push(benchmark);
        self
    }

    pub fn build(self) -> Driver<T> {
        Driver {
            options: self.options,
            loggers: self.loggers,
            benchmarks: self.benchmarks,
        }
    }
}

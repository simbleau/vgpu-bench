use std::path::Path;

use simplelog::SharedLogger;

use crate::{Benchmark, Driver, DriverOptions};

// Driver builder
pub struct DriverBuilder<'a> {
    pub options: DriverOptions<'a>,
    loggers: Vec<Box<dyn SharedLogger>>,
    benchmarks: Vec<Benchmark>,
    on_error_panic: bool,
}

impl<'a> DriverBuilder<'a> {
    pub fn new() -> Self {
        Self {
            options: DriverOptions::default(),
            loggers: Vec::new(),
            benchmarks: Vec::new(),
            on_error_panic: true,
        }
    }

    pub fn on_error_panic(mut self, should_panic: bool) -> Self {
        self.on_error_panic = should_panic;
        self
    }

    pub fn output_dir(mut self, output_dir: &'a Path) -> Self {
        self.options.output_dir = output_dir;
        self
    }

    pub fn logger(mut self, logger: Box<dyn SharedLogger>) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn add(mut self, benchmark: Benchmark) -> Self {
        self.benchmarks.push(benchmark);
        self
    }

    pub fn build(self) -> Driver<'a> {
        Driver {
            options: self.options,
            loggers: self.loggers,
            benchmarks: self.benchmarks,
            on_error_panic: self.on_error_panic,
        }
    }
}

use std::path::Path;

use simplelog::SharedLogger;

use crate::{Benchmark, Driver, DriverOptions, Measurable};

// Driver builder
pub struct DriverBuilder<T>
where
    T: Measurable,
{
    pub(crate) options: DriverOptions,
    pub(crate) loggers: Vec<Box<dyn SharedLogger>>,
    pub(crate) benchmarks: Vec<Benchmark<T>>,
    pub(crate) on_error_panic: bool,
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
            on_error_panic: true,
        }
    }

    pub fn on_error_panic(mut self, should_panic: bool) -> Self {
        self.on_error_panic = should_panic;
        self
    }

    pub fn output_dir(mut self, output_dir: &Path) -> Self {
        self.options.output_dir = output_dir.to_owned();
        self
    }

    pub fn benchmark_dir_name<S>(mut self, dir_name: S) -> Self
    where
        S: Into<String>,
    {
        self.options.benchmarks_dir =
            self.options.output_dir.join(dir_name.into());
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
            on_error_panic: self.on_error_panic,
        }
    }
}

use crate::dictionary::OUTPUT_DIR_NAME;
use simplelog::{CombinedLogger, SharedLogger};
use std::path::Path;

// Driver options - Read only
pub struct DriverOptions<'a> {
    pub output_dir: &'a Path,
}

impl Default for DriverOptions<'_> {
    fn default() -> Self {
        DriverOptions {
            output_dir: Path::new(OUTPUT_DIR_NAME),
        }
    }
}

// Driver fields
pub struct Driver<'a> {
    options: DriverOptions<'a>,
    loggers: Vec<Box<dyn SharedLogger>>,
    benchmarks: Vec<Box<dyn Fn(&DriverOptions)>>,
}

impl<'a> Driver<'a> {
    pub fn builder() -> DriverBuilder<'a> {
        DriverBuilder::new()
    }

    pub fn run(self) {
        // Initialize logger
        CombinedLogger::init(self.loggers).unwrap();

        // Run all benchmarks
        for func in self.benchmarks {
            func(&self.options);
        }
    }
}

// Driver builder
pub struct DriverBuilder<'a> {
    pub options: DriverOptions<'a>,
    loggers: Vec<Box<dyn SharedLogger>>,
    benchmarks: Vec<Box<dyn Fn(&DriverOptions)>>,
}

impl<'a> DriverBuilder<'a> {
    fn new() -> Self {
        Self {
            options: DriverOptions::default(),
            loggers: Vec::new(),
            benchmarks: Vec::new(),
        }
    }

    pub fn output_dir(mut self, output_dir: &'a Path) -> Self {
        self.options.output_dir = output_dir;
        self
    }

    pub fn logger(mut self, logger: Box<dyn SharedLogger>) -> Self {
        self.loggers.push(logger);
        self
    }

    pub fn add<F: Fn(&DriverOptions) + 'static>(mut self, f: F) -> Self {
        self.benchmarks.push(Box::new(f));
        self
    }

    pub fn build(self) -> Driver<'a> {
        Driver {
            options: self.options,
            loggers: self.loggers,
            benchmarks: self.benchmarks,
        }
    }
}

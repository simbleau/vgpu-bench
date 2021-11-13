use super::dictionary::*;
use crate::benchmarks::{Benchmark, BenchmarkFn};
use log::{error, info};
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
    benchmarks: Vec<Box<dyn Benchmark>>,
}

impl<'a> Driver<'a> {
    pub fn builder() -> DriverBuilder<'a> {
        DriverBuilder::new()
    }

    pub fn run(self) {
        // Initialize logger
        CombinedLogger::init(self.loggers).unwrap();
        info!("Logging started...");

        // Run all benchmarks
        for builder in self.benchmarks {
            if let Err(err) = builder.build().call(&self.options) {
                error!("Benchmark Failed: {}", err);
            }
        }
    }
}

// Driver builder
pub struct DriverBuilder<'a> {
    pub options: DriverOptions<'a>,
    loggers: Vec<Box<dyn SharedLogger>>,
    benchmarks: Vec<Box<dyn Benchmark>>,
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

    pub fn add<B>(mut self, benchmark: B) -> Self
    where
        B: Benchmark + 'static,
    {
        self.benchmarks.push(Box::new(benchmark));
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

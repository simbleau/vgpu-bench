use super::dictionary::*;
use crate::benchmarks::Benchmark;
use log::{error, info, trace};
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
    on_error_panic: bool,
}

impl<'a> Driver<'a> {
    pub fn builder() -> DriverBuilder<'a> {
        DriverBuilder::new()
    }

    pub fn run(self) {
        // Initialize logger
        CombinedLogger::init(self.loggers).unwrap();
        info!("logging started");

        // Build all benchmarks
        trace!("commencing driver building");
        let mut benchmarks = Vec::new();
        for builder in self.benchmarks {
            match builder.build() {
                Ok(b) => benchmarks.push(b),
                Err(e) => {
                    error!("benchmark build failed: {}", e);
                    if self.on_error_panic {
                        panic!("{}", e);
                    }
                }
            }
        }
        trace!("completed driver build");

        // Run all benchmarks
        trace!("commencing benchmarks");
        for benchmark in benchmarks {
            if let Err(e) = benchmark.call(&self.options) {
                error!("benchmark failed: {}", e);
                if self.on_error_panic {
                    panic!("{}", e);
                }
            }
        }
        trace!("completed benchmarks");
    }
}

// Driver builder
pub struct DriverBuilder<'a> {
    pub options: DriverOptions<'a>,
    loggers: Vec<Box<dyn SharedLogger>>,
    benchmarks: Vec<Box<dyn Benchmark>>,
    on_error_panic: bool,
}

impl<'a> DriverBuilder<'a> {
    fn new() -> Self {
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
            on_error_panic: self.on_error_panic,
        }
    }
}

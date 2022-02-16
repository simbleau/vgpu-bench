use crate::models::Benchmark;
use log::{error, trace};
use simplelog::{CombinedLogger, SharedLogger};
use std::path::{Path, PathBuf};

// Driver options - Read only
pub struct DriverOptions<'a> {
    output_dir: &'a Path,
}

impl Default for DriverOptions<'_> {
    fn default() -> Self {
        DriverOptions {
            output_dir: Path::new("output"),
        }
    }
}

impl DriverOptions<'_> {
    pub fn benchmark_dir(&self) -> PathBuf {
        self.output_dir.join("benchmarks")
    }
}

// Driver fields
pub struct Driver<'a> {
    options: DriverOptions<'a>,
    loggers: Vec<Box<dyn SharedLogger>>,
    benchmarks: Vec<Benchmark>,
    on_error_panic: bool,
}

impl<'a> Driver<'a> {
    pub fn builder() -> DriverBuilder<'a> {
        DriverBuilder::new()
    }

    pub fn run(self) {
        // Initialize logger
        if let Err(e) = CombinedLogger::init(self.loggers) {
            let err_msg =
                "Logged failed to initialize... Was it already initialized?";
            eprintln!("{err_msg}\n{e}");
        }
        trace!("logging initialized");

        // Run all benchmarks
        nvtx::mark("benchmark-stage");
        trace!("commencing benchmarks");
        for mut benchmark in self.benchmarks {
            nvtx::range_push(
                format!("benching {}", benchmark.metadata().name).as_str(),
            );
            let result = benchmark.run(&self.options);
            nvtx::range_pop();
            if let Err(e) = result {
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
    benchmarks: Vec<Benchmark>,
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

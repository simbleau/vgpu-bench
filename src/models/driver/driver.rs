use crate::{
    models::Benchmark, util, DriverBuilder, DriverOptions, Measurable, Result,
};
use log::{error, trace};
use simplelog::{CombinedLogger, SharedLogger};

// Driver fields
pub struct Driver<'a, T>
where
    T: Measurable,
{
    pub(crate) options: DriverOptions<'a>,
    pub(crate) loggers: Vec<Box<dyn SharedLogger>>,
    pub(crate) benchmarks: Vec<Benchmark<T>>,
    pub(crate) on_error_panic: bool,
}

impl<'a, T> Driver<'a, T>
where
    T: Measurable,
{
    pub fn builder() -> DriverBuilder<'a, T> {
        DriverBuilder::new()
    }

    pub fn run(self) -> Result<()> {
        // Initialize logger
        if let Err(e) = CombinedLogger::init(self.loggers) {
            let err_msg =
                "Logged failed to initialize... Was it already initialized?";
            eprintln!("{err_msg}\n{e}");
        }
        trace!("logging initialized");

        // Check conditions
        util::io::create_data_landing(self.options.output_dir)?;

        // Run all benchmarks
        nvtx::mark("benchmark-stage");
        trace!("commencing benchmarks");
        for mut benchmark in self.benchmarks {
            nvtx::range_push(
                format!("benching {}", benchmark.metadata().name).as_str(),
            );
            let result = benchmark.run(&self.options);
            nvtx::range_pop();
            match result {
                Ok(measurements) => {
                    println!("{:?}", measurements);
                }
                Err(e) => {
                    error!("benchmark failed: {}", e);
                    if self.on_error_panic {
                        panic!("{}", e);
                    }
                }
            }
        }
        trace!("completed benchmarks");
        Ok(())
    }
}

use log::{error, info, trace};
use std::collections::HashMap;

use crate::models::{
    Benchmark, BenchmarkBundle, DriverBuilder, DriverBundle, DriverOptions,
    DriverWriteMode, Measurable,
};
use crate::Result;
use crate::{log_assert, util};

// Driver fields
pub struct Driver<T>
where
    T: Measurable,
{
    pub(crate) options: DriverOptions,
    pub(crate) benchmarks: Vec<Benchmark<T>>,
}

impl<T> From<Benchmark<T>> for Driver<T>
where
    T: Measurable,
{
    fn from(benchmark: Benchmark<T>) -> Self {
        Driver::builder().add(benchmark).build()
    }
}

impl<T> Driver<T>
where
    T: Measurable,
{
    pub fn builder() -> DriverBuilder<T> {
        DriverBuilder::new()
    }

    pub fn run(self) -> Result<()> {
        let output_dir = self.options.output_dir.clone();
        let write_mode = self.options.write_mode().clone();

        let bundle = self.extract()?;

        // Check data landing
        trace!("preparing data landing");
        util::io::create_data_landing(&output_dir)?;
        match write_mode {
            DriverWriteMode::NoMash => {
                log_assert!(
                    util::io::dir_is_empty(&output_dir),
                    "Driver enforces output directory is empty: {dir}",
                    dir = output_dir.display()
                );
            }
            DriverWriteMode::Purge => util::io::dir_purge(&output_dir)?,
            DriverWriteMode::Relaxed => {}
        }
        trace!("landing ready");

        bundle.write(&output_dir)?;

        Ok(())
    }

    pub fn extract(self) -> Result<DriverBundle<T>> {
        // Create buffers
        let mut bundles: HashMap<String, BenchmarkBundle<T>> = HashMap::new();

        // Run all benchmarks
        nvtx::mark("benchmark-stage");
        trace!("commencing benchmarks");
        for mut benchmark in self.benchmarks {
            let benchmark_name = benchmark.metadata().name();
            info!("{benchmark_name}: commencing");
            let benchmark_result = benchmark.run(&self.options);
            match benchmark_result {
                Ok(bundle) => {
                    info!("{benchmark_name}: completed");
                    bundles.insert(benchmark_name.to_owned(), bundle);
                }
                Err(e) => {
                    error!("{benchmark_name} failed: {e}");
                    if self.options.on_error_continue {
                        trace!("continuing to next benchmark...")
                    } else {
                        panic!("{e}");
                    }
                }
            }
        }
        trace!("completed benchmarks");

        // Package bundle
        let bundle = DriverBundle {
            benchmark_bundles: bundles,
        };
        Ok(bundle)
    }
}

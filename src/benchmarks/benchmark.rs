use crate::driver::DriverOptions;
use anyhow::Result;
use erased_serde::Serialize;

pub struct Benchmark(
    Box<dyn FnOnce(&DriverOptions) -> Result<Vec<Box<dyn Serialize>>>>,
);

impl Benchmark {
    pub fn call(self, options: &DriverOptions) -> Result<()> {
        self.0(options)?;
        Ok(())
    }

    pub fn from<F>(function: F) -> Self
    where
        F: FnOnce(
                &DriverOptions,
            ) -> Result<Vec<Box<dyn erased_serde::Serialize>>>
            + 'static,
    {
        // do
        Benchmark(Box::new(function))
    }
}

pub trait BenchmarkBuilder {
    fn build(self) -> Benchmark;
}

use crate::driver::DriverOptions;
use anyhow::Result;
use erased_serde::Serialize;

pub struct Benchmark(Box<dyn FnOnce(&DriverOptions) -> Result<()>>);

impl Benchmark {
    pub fn call(self, options: &DriverOptions) -> Result<()> {
        self.0(options)?;
        Ok(())
    }

    pub fn from<F>(function: F) -> Self
    where
        F: FnOnce(&DriverOptions) -> Result<()> + 'static,
    {
        Benchmark(Box::new(function))
    }
}

pub trait BenchmarkBuilder {
    fn build(self) -> Benchmark;
}

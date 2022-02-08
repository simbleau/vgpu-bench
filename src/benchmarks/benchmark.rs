use crate::driver::DriverOptions;
use anyhow::Result;

pub struct BenchmarkFn(Box<dyn FnOnce(&DriverOptions) -> Result<()>>);

impl BenchmarkFn {
    pub fn call(self, options: &DriverOptions) -> Result<()> {
        Ok(self.0(options)?)
    }

    pub fn from<F>(function: F) -> Result<Self>
    where
        F: FnOnce(&DriverOptions) -> Result<()> + 'static,
    {
        Ok(BenchmarkFn(Box::new(function)))
    }
}

pub trait Benchmark: BenchmarkData + BenchmarkBuilder {}

pub trait BenchmarkData {
    fn name(&self) -> &'static str;
}

pub trait BenchmarkBuilder {
    fn build(self: Box<Self>) -> Result<BenchmarkFn>;
}

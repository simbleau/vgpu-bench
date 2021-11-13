use crate::driver::DriverOptions;
use anyhow::Result;

pub struct BenchmarkFn(Box<dyn FnOnce(&DriverOptions) -> Result<()>>);

impl BenchmarkFn {
    pub fn call(self, options: &DriverOptions) -> Result<()> {
        self.0(options)?;
        Ok(())
    }

    pub fn from<F>(function: F) -> Self
    where
        F: FnOnce(&DriverOptions) -> Result<()> + 'static,
    {
        BenchmarkFn(Box::new(function))
    }
}

pub trait Benchmark {
    fn build(self: Box<Self>) -> BenchmarkFn;
}

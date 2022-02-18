use crate::BenchmarkOptions;
use anyhow::Result;

pub struct BenchmarkFn(Box<dyn FnOnce(&BenchmarkOptions) -> Result<()>>);

impl BenchmarkFn {
    pub fn run(self, options: &BenchmarkOptions) -> Result<()> {
        Ok(self.0(options)?)
    }

    pub fn from<F>(func: F) -> Self
    where
        F: FnOnce(&BenchmarkOptions) -> Result<()> + 'static,
    {
        BenchmarkFn(Box::new(func))
    }
}

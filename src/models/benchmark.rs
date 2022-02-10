use crate::driver::DriverOptions;
use anyhow::Result;

pub struct BenchmarkFn(Box<dyn FnOnce(&DriverOptions) -> Result<()>>);

impl BenchmarkFn {
    pub fn call(self, options: &DriverOptions) -> Result<()> {
        Ok(self.0(options)?)
    }

    pub fn from<F>(func: F) -> Self
    where
        F: FnOnce(&DriverOptions) -> Result<()> + 'static,
    {
        BenchmarkFn(Box::new(func))
    }
}

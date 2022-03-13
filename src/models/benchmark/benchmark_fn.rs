use crate::{BenchmarkOptions, Measurements};
use crate::{Measurable, Result};

pub struct BenchmarkFn<T: Measurable>(
    Box<dyn FnOnce(&BenchmarkOptions) -> Result<Measurements<T>>>,
);

impl<T> BenchmarkFn<T>
where
    T: Measurable,
{
    pub fn run(self, options: &BenchmarkOptions) -> Result<Measurements<T>> {
        Ok(self.0(options)?)
    }

    pub fn from<F>(func: F) -> Self
    where
        F: FnOnce(&BenchmarkOptions) -> Result<Measurements<T>> + 'static,
    {
        BenchmarkFn(Box::new(func))
    }
}

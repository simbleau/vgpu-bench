use crate::models::{Measurable, Measurements};
use crate::Result;

/// Boxed dyn FnOnce type producing a Result<Measurable<T>>.
pub struct BenchmarkFn<T: Measurable>(
    Box<dyn FnOnce() -> Result<Measurements<T>>>,
);

/* One day in the future when "existential type aliases" exist, we can do:
impl<T, F> From<F> for BenchmarkFn<T>
where
    F: impl <for 'r> FnOnce(&'r BenchmarkOptions) -> Result<Measurements<T>> + 'static,
    T: Measurable,
{
    fn from(func: F) -> BenchmarkFn<T> {
        BenchmarkFn(Box::new(func))
    }
}
*/

impl<T, F> From<F> for BenchmarkFn<T>
where
    F: FnOnce() -> Result<Measurements<T>> + 'static,
    T: Measurable,
{
    fn from(func: F) -> BenchmarkFn<T> {
        BenchmarkFn(Box::new(func))
    }
}

impl<T> BenchmarkFn<T>
where
    T: Measurable,
{
    pub fn extract(self) -> Result<Measurements<T>> {
        Ok(self.0()?)
    }

    pub fn run<S>(self, name: S) -> Result<Measurements<T>>
    where
        S: Into<String>,
    {
        nvtx::range_push(
            format!("benching {name}", name = name.into()).as_str(),
        );
        let result = self.extract();
        nvtx::range_pop();
        result
    }

    pub fn new<F>(func: F) -> Self
    where
        F: FnOnce() -> Result<Measurements<T>> + 'static,
    {
        BenchmarkFn(Box::new(func))
    }
}

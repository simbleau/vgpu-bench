use crate::driver::DriverOptions;

pub struct Benchmark(
    Box<dyn Fn(&DriverOptions) -> Vec<Box<dyn erased_serde::Serialize>>>,
);

impl Benchmark {
    pub fn call(&self, options: &DriverOptions) {
        self.0(options);
    }

    pub fn from<F>(closure: F) -> Self
    where
        F: Fn(&DriverOptions) -> Vec<Box<dyn erased_serde::Serialize>>
            + 'static,
    {
        // do
        Benchmark(Box::new(closure))
    }
}

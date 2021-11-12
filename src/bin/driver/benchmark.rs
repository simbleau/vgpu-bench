use erased_serde::Serialize;

use crate::driver::DriverOptions;

pub struct Benchmark(
    pub fn(options: &DriverOptions) -> Vec<Box<dyn erased_serde::Serialize>>,
);

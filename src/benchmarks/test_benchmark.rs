use crate::driver::DriverOptions;

use super::Benchmark;

pub fn testy() -> Benchmark {
    Benchmark::from(|x| Vec::new())
}

pub fn testx(options: &DriverOptions) -> Vec<Box<dyn erased_serde::Serialize>> {
    Vec::new()
}

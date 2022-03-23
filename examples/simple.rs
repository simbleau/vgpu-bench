use vgpu_bench::macros::measurement;
use vgpu_bench::prelude::*;

#[measurement]
struct ExampleMeasurement {
    time: i32,
    amplitude: i32,
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    // Run driver
    Driver::from(Benchmark::from(BenchmarkFn::new(|_| {
        let mut measurements = Measurements::new();
        // Some real benchmarking would happen here
        for i in 0..10 {
            measurements.push(ExampleMeasurement {
                time: i,
                amplitude: i * i,
            });
        }
        // Benchmarking done!
        Ok(measurements)
    })))
    .run()
}

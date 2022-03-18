use vgpu_bench::macros::measurement;
use vgpu_bench::Benchmark;
use vgpu_bench::BenchmarkFn;
use vgpu_bench::BenchmarkOptions;
use vgpu_bench::Driver;
use vgpu_bench::Measurements;
use vgpu_bench::Result;

#[measurement]
struct ExampleMeasurement {
    time: i32,
    amplitude: i32,
}

pub fn main() -> Result<()> {
    Driver::from(Benchmark::from(BenchmarkFn::from(|_| {
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

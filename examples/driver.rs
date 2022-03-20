use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::macros::measurement;
use vgpu_bench::Benchmark;
use vgpu_bench::BenchmarkFn;
use vgpu_bench::BenchmarkMetadata;
use vgpu_bench::Driver;
use vgpu_bench::Measurements;

#[measurement]
struct ExampleMeasurement {
    time: i32,
    amplitude: i32,
}

pub fn main() {
    let metadata = BenchmarkMetadata::new("My Benchmark");
    let func = BenchmarkFn::new(|_| {
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
    });
    let benchmark = Benchmark::new(metadata, func);

    Driver::builder()
        .logger(TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .add(benchmark)
        .build()
        .run()
        .unwrap();
}

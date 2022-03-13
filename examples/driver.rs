use log::LevelFilter;
use serde::Serialize;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::Benchmark;
use vgpu_bench::Driver;
use vgpu_bench::Measurements;

pub fn main() {
    let benchmark = Benchmark::from("Benchmark-1", |_| {
        let mut measurements = Measurements::new();
        // Some real benchmarking would happen here
        #[derive(Serialize)]
        struct ExampleMeasurement {
            time: i32,
            amplitude: i32,
        }
        for i in 0..10 {
            measurements.push(ExampleMeasurement {
                time: i,
                amplitude: i * i,
            });
        }
        // Benchmarking done!
        Ok(measurements.into())
    });

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

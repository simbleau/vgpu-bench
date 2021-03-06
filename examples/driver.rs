use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::prelude::*;

#[measurement]
struct ExampleMeasurement {
    time: i32,
    amplitude: i32,
}

#[measurement]
struct ExampleMonitorMeasurement {
    value: i32,
}

// TODO: One day we should have #[monitor(name="Test", frequency=100)]
struct ExampleMonitor;
impl Monitor for ExampleMonitor {
    fn name(&self) -> &'static str {
        "Test"
    }

    fn frequency(&self) -> MonitorFrequency {
        MonitorFrequency::Hertz(100)
    }

    fn poll(&self) -> Result<Measurement> {
        Ok(Measurement::from(ExampleMonitorMeasurement { value: 5 }))
    }
}

pub fn main() {
    let metadata = BenchmarkMetadata::new("My Benchmark");
    let func: BenchmarkFn<ExampleMeasurement> = BenchmarkFn::new(|| {
        let mut measurements = Measurements::new();
        // Some real benchmarking would happen here
        for i in 0..10 {
            measurements.push(ExampleMeasurement {
                time: i,
                amplitude: i * i,
            });
            std::thread::sleep(std::time::Duration::from_secs_f64(0.1));
        }
        // Benchmarking done!
        Ok(measurements)
    });
    let benchmark = Benchmark::new(metadata, func).monitor(ExampleMonitor);

    vgpu_bench::util::logging::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )]);

    Driver::builder()
        .add(benchmark)
        .on_error_contune(true)
        .build()
        .run()
        .unwrap();
}

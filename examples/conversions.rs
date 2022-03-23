#![feature(duration_constants)]
use std::{thread, time::Duration};
use vgpu_bench::{monitors::CpuUtilizationMonitor, prelude::*};

#[measurement]
struct RenderTime {
    render_time_ms: u64,
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    let func = || {
        let mut measurements = Measurements::new();
        // Collect real measurements here...
        for i in 0..50 {
            let render_time_ms = (100.0 + 50.0 * (i as f32).sin()) as u64;
            measurements.push(RenderTime { render_time_ms });
            thread::sleep(Duration::from_millis(render_time_ms));
        }
        // Benchmarking done!
        Ok(measurements)
    };

    // Convert closure into GPU-annotated `BenchmarkFn`
    let benchmark_fn: BenchmarkFn<RenderTime> = func.into();
    // Create `Benchmark` from `BenchmarkFn`
    let benchmark: Benchmark<RenderTime> = Benchmark::from(benchmark_fn)
        .monitor(Box::new(CpuUtilizationMonitor {
            name: "CPU Utilization Monitor",
            frequency: MonitorFrequency::Hertz(1),
        }));
    // Convert `Benchmark` into `Driver`
    let driver: Driver<RenderTime> = benchmark.into();

    // Execute
    Ok(driver.run()?)
}

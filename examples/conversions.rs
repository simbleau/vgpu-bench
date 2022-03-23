#![feature(duration_constants)]
use std::{thread, time::Duration};
use vgpu_bench::{monitors::CpuUtilizationMonitor, prelude::*};

#[measurement]
struct RenderTime {
    render_time_ms: f32,
}

pub fn main() -> Result<()> {
    // Init logging
    vgpu_bench::util::logging::init_default();

    let closure = || {
        let mut measurements = Measurements::new();
        // Collect real measurements here...
        for i in 0..5 {
            let render_time_ms = 1.0 + 0.5 * (i as f32).sin();
            measurements.push(RenderTime { render_time_ms });
            thread::sleep(Duration::from_secs_f32(render_time_ms));
        }
        // Benchmarking done!
        Ok(measurements)
    };

    // Convert closure into GPU-annotated `BenchmarkFn`
    let benchmk_fn: BenchmarkFn<RenderTime> = closure.into();
    // Create `Benchmark` from `BenchmarkFn`
    let benchmark: Benchmark<RenderTime> =
        Benchmark::from(benchmk_fn).monitor(CpuUtilizationMonitor {
            name: "CPU Utilization Monitor",
            frequency: MonitorFrequency::Hertz(1),
        });
    // Convert `Benchmark` into `Driver`
    let driver: Driver<RenderTime> = benchmark.into();

    // Execute
    Ok(driver.run()?)
}

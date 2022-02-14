use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::driver::Driver;
use vgpu_bench::models::{
    BenchmarkFn, BenchmarkMetadata, MonitorFrequency, Unit,
};
use vgpu_bench::monitors::{CpuUtilizationMonitor, HeartbeatMonitor};

use std::thread;
use std::time::Duration;

pub fn main() {
    let benchmark_data = BenchmarkMetadata { name: "test" };
    let benchmark_fn = BenchmarkFn::from(|_| {
        thread::sleep(Duration::from_secs(5));
        Ok(())
    });

    let mut unit = Unit::new(benchmark_data, benchmark_fn);
    unit.monitors_mut().push(Box::new(HeartbeatMonitor::new(
        "Mon1",
        MonitorFrequency::Hertz(3),
    )));
    unit.monitors_mut()
        .push(Box::new(CpuUtilizationMonitor::default()));

    Driver::builder()
        .logger(TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .add(unit)
        .build()
        .run();

    // TODO display monitoring and results
}

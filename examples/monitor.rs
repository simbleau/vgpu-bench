use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use vgpu_bench::monitors::HeartbeatMonitor;
use vgpu_bench::Driver;
use vgpu_bench::{Benchmark, Monitor, MonitorFrequency};

use std::thread;
use std::time::Duration;

pub fn main() {
    let mut benchmark = Benchmark::from("Benchmark-1", |_| {
        // Some expensive operation...
        Ok(thread::sleep(Duration::from_secs(5)))
    });

    // Add monitors to the benchmark
    let mut monitors: Vec<Box<(dyn Monitor + Send + Sync + 'static)>> = vec![];
    // The 'Heartbeat' Monitor polls the (int) amount of heartbeats over time.
    // It can be used for time tracking but is just a placeholder here.
    monitors.push(Box::new(HeartbeatMonitor::new(
        "Monitor-1",
        MonitorFrequency::Hertz(1),
    )));
    monitors.push(Box::new(HeartbeatMonitor::new(
        "Monitor-2",
        MonitorFrequency::Hertz(2),
    )));
    benchmark.monitors_mut().extend(monitors);

    Driver::builder()
        .logger(TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .add(benchmark)
        .build()
        .run()
        .unwrap();
}

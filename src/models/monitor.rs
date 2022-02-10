use super::MonitorMetadata;
use std::time::Duration;

#[derive(Debug)]
pub enum Measurable {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Illegal,
    Uninitialized,
}

#[derive(Debug, Clone, Copy)]
pub enum MonitorFrequency {
    Hertz(usize),
    Duration(Duration),
}

impl MonitorFrequency {
    pub fn as_duration(&self) -> Duration {
        match self {
            MonitorFrequency::Hertz(hz) => {
                Duration::from_secs(1).div_f64(*hz as f64)
            }
            MonitorFrequency::Duration(dur) => *dur,
        }
    }
}

pub trait Monitor {
    fn metadata(&self) -> &MonitorMetadata;

    fn before(&mut self);

    fn tick(&mut self);

    fn poll(&self) -> Measurable;

    fn after(&mut self);
}

#[cfg(test)]
#[test]
fn test_monitor() {
    use log::LevelFilter;
    use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

    use super::BenchmarkFn;
    use crate::driver::Driver;
    use crate::models::benchmark_metadata::BenchmarkMetadata;
    use crate::models::unit::Unit;
    use crate::monitors::HeartbeatMonitor;
    use std::thread;
    use std::time::Duration;

    let benchmark_data = BenchmarkMetadata { name: "test" };
    let benchmark_fn = BenchmarkFn::from(|_| {
        thread::sleep(Duration::from_secs(5));
        Ok(())
    });

    let mut unit = Unit::new(benchmark_data, benchmark_fn);
    unit.monitors_mut().push(Box::new(HeartbeatMonitor::new(
        "Mon1",
        MonitorFrequency::Hertz(1),
    )));
    unit.monitors_mut().push(Box::new(HeartbeatMonitor::new(
        "Mon2",
        MonitorFrequency::Hertz(10),
    )));

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
}

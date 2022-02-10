use super::MonitorMetadata;
use std::time::Duration;

pub enum Measurable {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Illegal,
    Uninitialized,
}

pub enum MonitorFrequency {
    Hertz(usize),
    Duration(Duration),
}

pub trait Monitor {
    fn metadata(&self) -> &'static MonitorMetadata;

    fn before(&mut self);

    fn tick(&mut self);

    fn poll(&self) -> Measurable;

    fn after(&mut self);
}

#[cfg(test)]
#[test]
fn test_monitor() {
    use super::BenchmarkFn;
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

    let mut monitors = Vec::<Box<dyn Monitor + Send + Sync>>::new();
    monitors.push(Box::new(HeartbeatMonitor::default()));

    let _unit = Unit {
        data: &benchmark_data,
        func: &benchmark_fn,
        monitors,
    };
}

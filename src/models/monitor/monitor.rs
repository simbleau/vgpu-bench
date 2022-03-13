use crate::Measurement;
use crate::MonitorMetadata;
use crate::Result;
use std::time::Duration;

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

    fn on_start(&mut self);

    fn poll(&self) -> Result<Measurement>;

    fn on_stop(&mut self);
}

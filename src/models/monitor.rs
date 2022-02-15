use super::MonitorMetadata;
use anyhow::Result;
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

    fn on_start(&mut self);

    fn poll(&self) -> Result<Measurable>;

    fn on_stop(&mut self);
}

// TODO figure this out
/*
impl dyn Monitor + Sized {
    pub fn as_monitor(self: Box<Self>) -> Box<dyn Monitor> {
        Box::new(*self)
    }
}
*/

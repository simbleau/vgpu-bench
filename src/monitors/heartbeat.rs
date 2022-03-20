use anyhow::Result;
use serde::Serialize;
use std::time::Instant;

use crate::models::{Monitor, MonitorFrequency};
use crate::monitors::MonitorError;
use crate::Measurement;

#[derive(Serialize, Debug)]
struct HeartbeatMeasurement {
    beat: u32,
    elapsed_ns: u128,
}
unsafe impl Send for HeartbeatMeasurement {}
unsafe impl Sync for HeartbeatMeasurement {}

pub struct HeartbeatMonitor {
    beating: bool,
    beating_since: Option<Instant>,
}
unsafe impl Send for HeartbeatMonitor {}

impl HeartbeatMonitor {
    pub fn new<S: Into<String>>() -> Self {
        HeartbeatMonitor {
            beating: false,
            beating_since: None,
        }
    }
}
impl Default for HeartbeatMonitor {
    fn default() -> Self {
        Self {
            beating: false,
            beating_since: None,
        }
    }
}

impl Monitor for HeartbeatMonitor {
    fn name(&self) -> &'static str {
        "Heartbeat"
    }

    fn frequency(&self) -> MonitorFrequency {
        MonitorFrequency::Hertz(2)
    }

    fn on_start(&mut self) {
        self.beating = true;
        self.beating_since = Some(Instant::now());
    }

    fn poll(&self) -> Result<Measurement> {
        match self.beating {
            true => {
                let elapsed = Instant::now().duration_since(
                    self.beating_since.expect("Was this monitor initialized?"),
                );
                let beat = elapsed
                    .div_duration_f64(self.frequency().as_duration())
                    as u32;
                let heartbeat_measurement = HeartbeatMeasurement {
                    beat,
                    elapsed_ns: elapsed.as_nanos(),
                };
                Ok(Measurement::from(heartbeat_measurement))
            }
            false => Err(MonitorError::Polling(format!(
                "{name} is not beating. Was this monitor initialized?",
                name = self.name()
            ))
            .into()),
        }
    }

    fn on_stop(&mut self) {
        self.beating = false;
    }
}

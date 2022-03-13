use anyhow::Result;
use serde::Serialize;
use std::time::Instant;

use crate::models::{Monitor, MonitorFrequency, MonitorMetadata};
use crate::monitors::heartbeat::MonitorFrequency::Hertz;
use crate::monitors::MonitorError;
use crate::{util, Measurement};

#[derive(Serialize)]
struct HeartbeatMeasurement {
    beat: u32,
    elapsed_ns: u128,
}

pub struct HeartbeatMonitor {
    metadata: MonitorMetadata,
    beating: bool,
    beating_since: Option<Instant>,
}
unsafe impl Send for HeartbeatMonitor {}

impl HeartbeatMonitor {
    pub fn new<S: Into<String>>(name: S, frequency: MonitorFrequency) -> Self {
        HeartbeatMonitor {
            metadata: MonitorMetadata {
                name: name.into(),
                frequency,
            },
            beating: false,
            beating_since: None,
        }
    }
}
impl Default for HeartbeatMonitor {
    fn default() -> Self {
        Self {
            metadata: MonitorMetadata {
                name: "Heartbeat Monitor".to_string(),
                frequency: Hertz(1),
            },
            beating: false,
            beating_since: None,
        }
    }
}

impl Monitor for HeartbeatMonitor {
    fn metadata(&self) -> &MonitorMetadata {
        &self.metadata
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
                    .div_duration_f64(self.metadata.frequency.as_duration())
                    as u32;
                let heartbeat_measurement = HeartbeatMeasurement {
                    beat,
                    elapsed_ns: elapsed.as_nanos(),
                };
                // TODO make this cleaner. Bandaid fix
                let measurement = Measurement {
                    inner: Box::new(util::convert::erase(
                        heartbeat_measurement,
                    )),
                };
                Ok(measurement)
            }
            false => Err(MonitorError::Polling(format!(
                "{name} is not beating. Was this monitor initialized?",
                name = self.metadata.name
            ))
            .into()),
        }
    }

    fn on_stop(&mut self) {
        self.beating = false;
    }
}

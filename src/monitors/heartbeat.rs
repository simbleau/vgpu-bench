use std::time::Instant;

use crate::models::{Measurable, Monitor, MonitorFrequency, MonitorMetadata};
use crate::monitors::heartbeat::MonitorFrequency::Hertz;

pub struct HeartbeatMonitor {
    data: MonitorMetadata,
    beating: bool,
    beating_since: Option<Instant>,
}
unsafe impl Send for HeartbeatMonitor {}

impl HeartbeatMonitor {
    pub fn new<S: Into<String>>(name: S, frequency: MonitorFrequency) -> Self {
        HeartbeatMonitor {
            data: MonitorMetadata {
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
            data: MonitorMetadata {
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
        &self.data
    }

    fn on_init(&mut self) {
        self.beating = true;
        self.beating_since = Some(Instant::now());
    }

    fn poll(&self) -> Measurable {
        match self.beating {
            true => {
                let elapsed = Instant::now().duration_since(
                    self.beating_since.expect("Was this monitor initialized?"),
                );
                let beats =
                    elapsed.div_duration_f64(self.data.frequency.as_duration());
                Measurable::Integer(beats as i64)
            }
            false => Measurable::Illegal,
        }
    }

    fn on_destroy(&mut self) {
        self.beating = false;
    }
}

use crate::models::{Measurable, Monitor, MonitorFrequency, MonitorMetadata};
use crate::monitors::heartbeat::Heartbeat::{Beat1, Beat2};
use crate::monitors::heartbeat::MonitorFrequency::Hertz;

pub enum Heartbeat {
    Beat1,
    Beat2,
}

pub struct HeartbeatMonitor {
    data: MonitorMetadata,
    beating: bool,
    beat: Heartbeat,
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
            beat: Beat1,
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
            beat: Heartbeat::Beat1,
        }
    }
}

impl Monitor for HeartbeatMonitor {
    fn metadata(&self) -> &MonitorMetadata {
        &self.data
    }

    fn before(&mut self) {
        self.beating = true;
    }

    fn tick(&mut self) {
        if self.beating {
            match self.beat {
                Beat1 => self.beat = Beat2,
                Beat2 => self.beat = Beat1,
            }
        }
    }

    fn poll(&self) -> Measurable {
        match self.beating {
            true => match self.beat {
                Beat1 => Measurable::Integer(0),
                Beat2 => Measurable::Integer(1),
            },
            false => Measurable::Illegal,
        }
    }

    fn after(&mut self) {
        self.beating = false;
    }
}

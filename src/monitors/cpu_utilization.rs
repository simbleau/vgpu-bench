use crate::models::{Measurable, Monitor, MonitorFrequency, MonitorMetadata};
use std::time::Duration;

pub struct CpuUtilizationMonitor {
    metadata: MonitorMetadata,
}
unsafe impl Send for CpuUtilizationMonitor {}

impl Default for CpuUtilizationMonitor {
    fn default() -> Self {
        Self {
            metadata: MonitorMetadata {
                name: String::from("CPU Utilization"),
                frequency: MonitorFrequency::Duration(Duration::from_secs_f64(
                    0.25,
                )),
            },
        }
    }
}

impl Monitor for CpuUtilizationMonitor {
    fn metadata(&self) -> &MonitorMetadata {
        &self.metadata
    }

    fn on_init(&mut self) {}

    fn poll(&self) -> Measurable {
        todo!()
    }

    fn on_destroy(&mut self) {}
}

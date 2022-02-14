use anyhow::Result;
use systemstat::{Platform, System};

use crate::models::{Measurable, Monitor, MonitorFrequency, MonitorMetadata};
use crate::monitors::cpu_utilization::MonitorFrequency::Hertz;
use std::{thread, time::Duration};

pub struct MemoryUtilizationMonitor {
    metadata: MonitorMetadata,
}
unsafe impl Send for MemoryUtilizationMonitor {}

impl Default for MemoryUtilizationMonitor {
    fn default() -> Self {
        Self {
            metadata: MemoryUtilizationMonitor {
                name: String::from("Memory Utilization"),
                frequency: Hertz(100),
            },
        }
    }
}

impl Monitor for MemoryUtilizationMonitor {
    fn metadata(&self) -> &MonitorMetadata {
        &self.metadata
    }

    fn on_start(&mut self) {}

    fn poll(&self) -> Result<Measurable> {
        let sys = System::new();
        let memory = sys.memory()?;
        let utilization =
            (mem.free.as_u64() as f64 / mem.total.as_u64() as f64);
        Ok(Measurable::Float(utilization))
    }

    fn on_stop(&mut self) {}
}

use anyhow::Result;
use systemstat::{Platform, System};

use crate::models::{Measurable, Monitor, MonitorFrequency, MonitorMetadata};
use crate::monitors::cpu_utilization::MonitorFrequency::Hertz;
use std::{thread, time::Duration};

pub struct CpuUtilizationMonitor {
    metadata: MonitorMetadata,
}
unsafe impl Send for CpuUtilizationMonitor {}

impl Default for CpuUtilizationMonitor {
    fn default() -> Self {
        Self {
            metadata: MonitorMetadata {
                name: String::from("CPU Utilization"),
                frequency: Hertz(1),
            },
        }
    }
}

impl Monitor for CpuUtilizationMonitor {
    fn metadata(&self) -> &MonitorMetadata {
        &self.metadata
    }

    fn on_start(&mut self) {}

    fn poll(&self) -> Result<Measurable> {
        let sys = System::new();
        let load_aggregate = sys.cpu_load_aggregate()?;
        thread::sleep(Duration::from_secs_f64(0.75));
        let load = load_aggregate.done()?;
        Ok(Measurable::Float(1.0 - load.idle as f64))
    }

    fn on_stop(&mut self) {}
}

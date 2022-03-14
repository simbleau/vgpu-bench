use anyhow::Result;
use serde::Serialize;
use systemstat::{Platform, System};

use crate::models::{Monitor, MonitorFrequency, MonitorMetadata};
use crate::monitors::cpu_utilization::MonitorFrequency::Hertz;
use crate::Measurement;
use std::{thread, time::Duration};

#[derive(Serialize, Debug)]
struct CpuMeasurement {
    idle: f32,
    interrupt: f32,
    nice: f32,
    system: f32,
    user: f32,
}
unsafe impl Send for CpuMeasurement {}
unsafe impl Sync for CpuMeasurement {}
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

    fn poll(&self) -> Result<Measurement> {
        let sys = System::new();
        let load_aggregate = sys.cpu_load_aggregate()?;
        thread::sleep(Duration::from_secs_f64(0.75));
        let load = load_aggregate.done()?;
        let cpu_measurement = CpuMeasurement {
            idle: load.idle,
            interrupt: load.interrupt,
            nice: load.nice,
            system: load.system,
            user: load.user,
        };
        Ok(Measurement::from(cpu_measurement))
    }

    fn on_stop(&mut self) {}
}

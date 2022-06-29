use serde::Serialize;
use std::{thread, time::Duration};
use systemstat::{Platform, System};

use crate::models::{Measurement, Monitor, MonitorFrequency};
use crate::Result;

/// Type for CPU relevant metrics such as:
/// + IDLE/Hang-time
/// + Interrupt Count
/// +  Nice(?)
/// + System-Calls / Execution Time
/// + User Space Execution Time
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
    pub name: &'static str,
    pub frequency: MonitorFrequency,
}
unsafe impl Send for CpuUtilizationMonitor {}

impl Monitor for CpuUtilizationMonitor {
    fn name(&self) -> &'static str {
        self.name
    }

    fn frequency(&self) -> MonitorFrequency {
        self.frequency
    }

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
}

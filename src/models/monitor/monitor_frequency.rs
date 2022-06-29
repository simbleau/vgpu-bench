use std::time::Duration;

/// Type for defining how many times a monitor should execute and it's equivalent duration.
/// The Duration is defined as T = 1 second / f where f is defined in Hz.
#[derive(Debug, Clone, Copy)]
pub enum MonitorFrequency {
    Hertz(usize),
    Duration(Duration),
}

impl MonitorFrequency {
    
    /// Generates a Duration typefor the Monitors to execute.
    pub fn as_duration(&self) -> Duration {
        match self {
            MonitorFrequency::Hertz(hz) => {
                Duration::from_secs(1).div_f64(*hz as f64)
            }
            MonitorFrequency::Duration(dur) => *dur,
        }
    }
}

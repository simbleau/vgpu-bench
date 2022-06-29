use crate::models::{Measurement, MonitorFrequency};
use crate::Result;

// Alias for a Sync/Send compliant type
pub trait Monitor: Send + Sync {
    fn name(&self) -> &'static str;

    fn frequency(&self) -> MonitorFrequency;

    fn on_start(&mut self) {
        // Do nothing
    }

    fn poll(&self) -> Result<Measurement>;

    fn on_stop(&mut self) {
        // Do nothing
    }
}

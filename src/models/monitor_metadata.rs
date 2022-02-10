use crate::models::MonitorFrequency;

pub struct MonitorMetadata {
    pub name: &'static str,
    pub frequency: MonitorFrequency,
}

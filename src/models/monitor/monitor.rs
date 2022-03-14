use crate::Measurement;
use crate::MonitorMetadata;
use crate::Result;

pub trait Monitor {
    fn metadata(&self) -> &MonitorMetadata;

    fn on_start(&mut self);

    fn poll(&self) -> Result<Measurement>;

    fn on_stop(&mut self);
}

use serde::{Serialize, Serializer};

use crate::{util, Measurable};

pub struct Measurement {
    pub inner: Box<dyn erased_serde::Serialize>,
}
unsafe impl Send for Measurement {}
unsafe impl Sync for Measurement {}

impl Serialize for Measurement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(self.inner.serialize(serializer)?)
    }
}

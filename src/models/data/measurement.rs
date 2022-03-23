use serde::{Serialize, Serializer};
use std::fmt;

use crate::models::Measurable;

pub struct Measurement {
    measurable: Box<dyn erased_serde::Serialize>,
}
unsafe impl Send for Measurement {}
unsafe impl Sync for Measurement {}

impl Measurement {
    pub fn from(measurable: impl Measurable) -> Self {
        Measurement {
            measurable: Box::new(measurable),
        }
    }
}

impl<T: Measurable> From<Box<T>> for Measurement {
    fn from(item: Box<T>) -> Self {
        Measurement::from(*item)
    }
}

impl fmt::Debug for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Measurement").finish()
    }
}

impl Serialize for Measurement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(self.measurable.serialize(serializer)?)
    }
}

use serde::{Serialize, Serializer};
use std::fmt;

use crate::models::Measurable;

// Trait Object that implements serde Serialization. 
pub struct Measurement {
    measurable: Box<dyn erased_serde::Serialize>,
}

/// Implement a thread-unsafe Send trait for Measurement.
unsafe impl Send for Measurement {}
/// Implement a thread-unsafe Sync trait for Measurement.
unsafe impl Sync for Measurement {}

impl Measurement {

    /// Create a Measurement Struct from a Measurable type.
    pub fn from(measurable: impl Measurable) -> Self {
        Measurement {
            measurable: Box::new(measurable),
        }
    }
}

/// Create a Measurement from a Boxed type.
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

use serde::Serialize;
use std::fmt::Debug;

/// Type Alias for a thread-safe static lifetime struct that may be  serde Serializable.
pub trait Measurable = Serialize + Debug + Send + Sync + 'static;

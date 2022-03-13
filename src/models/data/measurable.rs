use serde::Serialize;
use std::fmt::Debug;

pub trait Measurable = Serialize + Debug + Send + Sync + 'static;

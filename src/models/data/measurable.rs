use serde::Serialize;

pub trait Measurable = Serialize + Send + Sync;

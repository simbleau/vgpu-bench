use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Measurable {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Illegal,
    Uninitialized,
}

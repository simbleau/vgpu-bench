use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct TimeResult {
    pub tessellator: String,
    pub filename: String,
    pub init_time: i32,
    pub tess_time: i32,
}

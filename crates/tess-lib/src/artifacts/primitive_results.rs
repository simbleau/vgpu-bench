use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct PrimitiveTimeResult {
    pub tessellator: String,
    pub primitive: String,
    pub init_time: i32,
    pub tess_time: i32,
}

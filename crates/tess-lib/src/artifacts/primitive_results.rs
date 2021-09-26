use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct PrimitiveTimeResult {
    pub tessellator: String,
    pub primitive: String,
    pub amount: u32,
    pub init_time: u128,
    pub tess_time: u128,
}

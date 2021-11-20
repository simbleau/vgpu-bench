use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NaiveSVGFileRenderTime {
    pub tessellator: String,
    pub filename: String,
    pub triangles: u32,
    pub frame: u32,
    pub frame_time: u128,
}

#[derive(Debug, Serialize)]
pub struct NaivePrimitiveRenderTime {
    pub tessellator: String,
    pub primitive: String,
    pub amount: u32,
    pub triangles: u32,
    pub frame: u32,
    pub frame_time: u128,
}

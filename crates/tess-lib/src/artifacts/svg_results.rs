use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct SVGProfileResult {
    pub tessellator: String,
    pub filename: String,
    pub vertices: u32,
    pub indices: u32,
}

#[derive(Debug, Serialize)]
pub struct SVGTimeResult {
    pub tessellator: String,
    pub filename: String,
    pub init_time: u32,
    pub tess_time: u32,
}

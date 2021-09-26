use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct SVGProfileResult {
    pub tessellator: String,
    pub filename: String,
    pub vertices: i32,
    pub indices: i32,
}

#[derive(Debug, Serialize)]
pub struct SVGTimeResult {
    pub tessellator: String,
    pub filename: String,
    pub init_time: i32,
    pub tess_time: i32,
}

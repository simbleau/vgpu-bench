use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct SVGProfileResult {
    pub tessellator: String,
    pub filename: String,
    pub vertices: i32,
    pub indices: i32,
}

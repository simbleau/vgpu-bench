use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TessellationResult {
    pub tessellator: String,
    pub filename: String,
    pub vertices: i32,
    pub indices: i32,
    pub init_time: i32,
    pub tess_time: i32,
}

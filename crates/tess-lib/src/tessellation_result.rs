use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TessellationResult {
    pub tessellator: String,
    pub filename: String,
    pub init_time: String,
    pub tess_time: String,
}

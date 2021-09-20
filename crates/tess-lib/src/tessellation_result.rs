use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TessellationResult {
    pub tessellator: String,
    pub filename: String,
    pub prep_time: String,
    pub tess_time: String,
}

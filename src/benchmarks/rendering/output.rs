use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileRenderTime {
    pub filename: String,
    pub frame: usize,
    pub frame_time: u128,
}

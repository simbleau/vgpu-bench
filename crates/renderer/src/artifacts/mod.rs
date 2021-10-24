use std::time::Duration;
pub mod types;

#[derive(Debug)]
pub struct RenderTimeResult {
    pub triangles: u32,
    pub frame_times: Vec<Duration>,
}

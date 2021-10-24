use std::time::Duration;

#[derive(Debug)]
pub struct RenderTimeResult {
    pub triangles: u32,
    pub frame_times: Vec<Duration>,
}

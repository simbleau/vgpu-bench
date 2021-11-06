use std::time::Duration;
pub mod types;

#[derive(Debug)]
pub struct RenderTimeResult {
    pub frame_times: Vec<Duration>,
}

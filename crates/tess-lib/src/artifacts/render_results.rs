use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FlattenedRenderResult {
    pub triangles: u32,
    pub frame_times: Vec<Duration>,
}

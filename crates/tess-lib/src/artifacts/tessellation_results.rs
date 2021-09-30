use std::time::Duration;

use serde::Serialize;

use crate::renderer::types::{GpuPrimitive, GpuTransform, GpuVertex};
#[derive(Debug, Serialize)]
pub struct TessellationProfile {
    pub vertices: u32,
    pub indices: u32,
}

#[derive(Debug)]
pub struct TessellationData {
    pub vertices: Vec<GpuVertex>,
    pub indices: Vec<u32>,
    pub transforms: Vec<GpuTransform>,
    pub primitives: Vec<GpuPrimitive>,
}

#[derive(Debug, Serialize)]
pub struct TessellationTimeResult {
    pub init_time: Duration,
    pub tess_time: Duration,
}

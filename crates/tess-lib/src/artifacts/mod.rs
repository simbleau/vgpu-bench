pub mod types;
use self::types::{GpuPrimitive, GpuTransform, GpuVertex};
use std::time::Duration;

#[derive(Debug)]
pub struct TessellationProfile {
    pub vertices: u32,
    pub indices: u32,
    pub triangles: u32,
}

#[derive(Debug)]

pub struct TessellationData {
    pub vertices: Vec<GpuVertex>,
    pub indices: Vec<u32>,
    pub transforms: Vec<GpuTransform>,
    pub primitives: Vec<GpuPrimitive>,
}

#[derive(Debug)]
pub struct TessellationTimeResult {
    pub init_time: Duration,
    pub tess_time: Duration,
}

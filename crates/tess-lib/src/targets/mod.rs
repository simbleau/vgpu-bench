mod svg_document;
mod svg_file;

use std::time::Duration;

pub use svg_document::SVGDocument;
pub use svg_file::SVGFile;

use crate::{
    renderer::state::{GpuPrimitive, GpuTransform, GpuVertex},
    Tessellator,
};

pub struct TessellationProfile {
    pub vertices: i32,
    pub indices: i32,
}

pub struct TessellationData {
    pub mesh: lyon::lyon_tessellation::VertexBuffers<GpuVertex, u32>,
    pub transforms: Vec<GpuTransform>,
    pub primitives: Vec<GpuPrimitive>,
}

pub struct TessellationTimeResult {
    pub init_time: Duration,
    pub tess_time: Duration,
}
pub trait TessellationTarget {
    fn get_data(&self, t: Box<&mut dyn Tessellator>) -> TessellationProfile;
    fn time(&mut self, t: Box<&mut dyn Tessellator>) -> TessellationTimeResult;
}

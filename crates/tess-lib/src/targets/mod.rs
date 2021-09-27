mod svg_document;
mod svg_file;

use std::time::Duration;

pub use svg_document::SVGDocument;
pub use svg_file::SVGFile;

use crate::Tessellator;

pub struct TessellationProfile {
    pub vertices: i32,
    pub indices: i32,
}

pub struct TessellationTimeResult {
    pub init_time: Duration,
    pub tess_time: Duration,
}
pub trait TessellationTarget {
    fn get_data(&self, t: Box<&mut dyn Tessellator>) -> TessellationProfile;
    fn time(&mut self, t: Box<&mut dyn Tessellator>) -> TessellationTimeResult;
}

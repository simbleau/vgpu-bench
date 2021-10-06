mod svg_document;
mod svg_file;

pub use svg_document::SVGDocument;
pub use svg_file::SVGFile;

use crate::{
    artifacts::{RenderTimeResult, TessellationProfile, TessellationTimeResult},
    backends::Tessellator,
    renderer,
};

pub trait TessellationTarget {
    fn get_data(&self, t: &mut dyn Tessellator) -> TessellationProfile;
    fn time(&mut self, t: &mut dyn Tessellator) -> TessellationTimeResult;
    fn time_render(
        &mut self,
        t: &mut dyn Tessellator,
        frames: usize,
    ) -> renderer::error::Result<RenderTimeResult>;
}

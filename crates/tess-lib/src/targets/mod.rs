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
    fn get_data(
        &self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationProfile, Box<dyn std::error::Error>>;

    fn time(
        &mut self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationTimeResult, Box<dyn std::error::Error>>;

    fn time_render(
        &mut self,
        t: &mut dyn Tessellator,
        frames: usize,
    ) -> renderer::error::Result<RenderTimeResult>;
}

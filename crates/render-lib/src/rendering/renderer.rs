use tess_lib::targets::SVGDocument;

use crate::artifacts::RenderTimeResult;
pub trait Renderer {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn stage(&mut self, svg: &mut SVGDocument) -> Result<(), Box<dyn std::error::Error>>;

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult, Box<dyn std::error::Error>>;
}

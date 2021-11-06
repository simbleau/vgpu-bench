use crate::artifacts::RenderTimeResult;
use crate::error::Result;
use crate::targets::SVGDocument;
pub trait Renderer {
    fn init(&mut self) -> Result<()>;

    fn stage(&mut self, svg: &SVGDocument) -> Result<()>;

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult>;
}

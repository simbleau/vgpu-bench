use artifacts::RenderTimeResult;
use targets::SVGDocument;

pub mod artifacts;
pub mod targets;

type AnyError = Box<dyn std::error::Error>;
pub trait Renderer {
    fn init(&mut self) -> Result<(), AnyError>;

    fn stage(&mut self, svg: &SVGDocument) -> Result<(), AnyError>;

    fn render(&mut self, frames: u64) -> Result<RenderTimeResult, AnyError>;
}

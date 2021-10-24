use renderer::{artifacts::RenderTimeResult, targets::SVGDocument};
pub mod rendering;
pub mod tessellation;

pub trait Renderer {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn stage(&mut self, svg: &mut SVGDocument) -> Result<(), Box<dyn std::error::Error>>;

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult, Box<dyn std::error::Error>>;
}

// TODO: Option builder for analysis.

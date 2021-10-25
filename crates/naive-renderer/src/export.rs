use renderer::{artifacts::RenderTimeResult, targets::SVGDocument, Renderer};
use tess_lib::{
    backends::{LyonTessellator, Tessellator},
    targets::SVGTarget,
};

use crate::TriangleRenderer;

// Wrapper
pub struct NaiveRenderer {
    renderer: TriangleRenderer,
    backend: Box<dyn Tessellator>,
}

impl NaiveRenderer {
    pub fn new() -> Self {
        let renderer = TriangleRenderer::new();
        let backend = Box::new(LyonTessellator::new());

        NaiveRenderer { renderer, backend }
    }
}

impl Renderer for NaiveRenderer {
    fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(()) // Renderer has no initialization
    }

    fn stage(&mut self, svg: &SVGDocument) -> Result<(), Box<dyn std::error::Error>> {
        let tessellator = &mut *self.backend;
        // Convert to target
        let x = svg.clone();
        let target = SVGTarget::from(x);
        Ok(self.renderer.init_with_svg(tessellator, &target)?)
    }

    fn render(&mut self, frames: u64) -> Result<RenderTimeResult, Box<dyn std::error::Error>> {
        Ok(self.renderer.time(frames)?)
    }
}

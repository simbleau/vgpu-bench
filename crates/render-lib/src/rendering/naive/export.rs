use crate::{artifacts::RenderTimeResult, rendering::Renderer};
use tess_lib::{
    backends::{LyonTessellator, Tessellator},
    targets::SVGDocument,
};

use super::renderer::TriangleRenderer;

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

    fn stage(&mut self, svg: &mut SVGDocument) -> Result<(), Box<dyn std::error::Error>> {
        let tessellator = &mut *self.backend;
        Ok(self.renderer.init_with_svg(tessellator, svg)?)
    }

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult, Box<dyn std::error::Error>> {
        Ok(self.renderer.time(frames)?)
    }
}

use renderer::RendererError::RustLibraryError;
use renderer::{
    artifacts::RenderTimeResult, rust::Renderer, targets::SVGDocument, Result,
};
use tessellation_util::backends::{LyonTessellator, Tessellator};

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
    fn init(&mut self) -> Result<()> {
        Ok(()) // Renderer has no initialization
    }

    fn stage(&mut self, svg: &SVGDocument) -> Result<()> {
        Ok(self
            .renderer
            .init_with_svg(self.backend.as_mut(), svg)
            .map_err(|err| RustLibraryError(Box::new(err)))?)
    }

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult> {
        Ok(self
            .renderer
            .time(frames)
            .map_err(|err| RustLibraryError(Box::new(err)))?)
    }
}

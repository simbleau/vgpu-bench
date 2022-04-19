use renderer::RendererError::RustLibraryError;
use renderer::{
    artifacts::RenderTimeResult, targets::SVGDocument, Renderer, Result,
};
use svg_tessellation_renderer::TriangleRenderer;

// Wrapper
pub struct Renderer1 {
    renderer: TriangleRenderer,
}

impl Renderer1 {
    pub fn new() -> Self {
        let renderer = TriangleRenderer::new();
        Renderer1 { renderer }
    }
}

impl Renderer for Renderer1 {
    fn init(&mut self) -> Result<()> {
        Ok(()) // Renderer has no initialization
    }

    fn stage(&mut self, svg: &SVGDocument) -> Result<()> {
        Ok(self
            .renderer
            .init_with_svg(svg.content())
            .map_err(|err| RustLibraryError(Box::new(err)))?)
    }

    fn render(&mut self, frames: usize) -> Result<RenderTimeResult> {
        Ok(self
            .renderer
            .time(frames)
            .map(|frame_times| RenderTimeResult { frame_times })
            .map_err(|err| RustLibraryError(Box::new(err)))?)
    }
}

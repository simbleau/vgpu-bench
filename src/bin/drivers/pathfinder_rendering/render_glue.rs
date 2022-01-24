use pathfinder_vgpu_glue::PathfinderRenderer;
use renderer::{artifacts::RenderTimeResult, targets::SVGDocument, Renderer};
use std::path::PathBuf;

pub struct PathfinderImpl {
    pathfinder: PathfinderRenderer,
    asset: PathBuf,
}

impl PathfinderImpl {
    pub fn new<P: Into<PathBuf>>(asset: P) -> Self {
        PathfinderImpl {
            pathfinder: PathfinderRenderer::new(),
            asset: asset.into(),
        }
    }
}

impl Renderer for PathfinderImpl {
    fn init(&mut self) -> renderer::Result<()> {
        self.pathfinder.init(self.asset.clone());
        Ok(())
    }

    fn stage(&mut self, _svg: &SVGDocument) -> renderer::Result<()> {
        Ok(())
    }

    fn render(&mut self, frames: usize) -> renderer::Result<RenderTimeResult> {
        let pathfinder = &mut self.pathfinder;
        let result = pathfinder.render(frames);
        let rt = RenderTimeResult {
            frame_times: result,
        };
        Ok(rt)
    }
}

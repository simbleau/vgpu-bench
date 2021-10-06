use crate::renderer;
use crate::targets::{SVGFile, TessellationProfile, TessellationTarget, TessellationTimeResult};
use crate::{artifacts::RenderTimeResult, backends::Tessellator, renderer::Renderer};
use std::time::Instant;

pub struct SVGDocument {
    pub content: String,
}

impl From<&SVGFile> for SVGDocument {
    fn from(item: &SVGFile) -> Self {
        let source = std::fs::read(item.path.clone()).unwrap();
        SVGDocument {
            content: String::from_utf8(source).unwrap(),
        }
    }
}

impl<T> From<T> for SVGDocument
where
    T: Into<String>,
{
    fn from(item: T) -> Self {
        SVGDocument {
            content: item.into(),
        }
    }
}

impl TessellationTarget for SVGDocument {
    fn get_data(&self, t: &mut dyn Tessellator) -> TessellationProfile {
        t.init(&self);
        t.tessellate().unwrap()
    }

    fn time(&mut self, t: &mut dyn Tessellator) -> TessellationTimeResult {
        // Time pre-processing
        let t1 = Instant::now();
        t.init(&self);
        let t2 = Instant::now();
        let dur1 = t2.duration_since(t1);

        // Time the tessellation
        let t1 = Instant::now();
        t.tessellate().unwrap();
        let t2 = Instant::now();
        let dur2 = t2.duration_since(t1);

        // Return duration passed
        TessellationTimeResult {
            init_time: dur1,
            tess_time: dur2,
        }
    }

    fn time_render(
        &mut self,
        t: &mut dyn Tessellator,
        frames: usize,
    ) -> renderer::error::Result<RenderTimeResult> {
        let mut r = Renderer::new();
        r.init_with_svg(t, &self)?;

        Ok(r.time(frames)?)
    }
}

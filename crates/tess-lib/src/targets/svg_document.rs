use std::time::Instant;

use crate::Tessellator;

use super::{SVGFile, TessellationProfile, TessellationTarget, TessellationTimeResult};
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
    fn get_data(&self, t: Box<&mut dyn Tessellator>) -> TessellationProfile {
        t.init(&self);
        t.tessellate().unwrap()
    }

    fn time(&mut self, t: Box<&mut dyn Tessellator>) -> TessellationTimeResult {
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
}

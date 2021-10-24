use crate::backends::Tessellator;
use crate::targets::{TessellationProfile, TessellationTarget, TessellationTimeResult};
use renderer::targets::SVGDocument;
use std::time::Instant;

pub struct SVGTarget(SVGDocument);

impl SVGTarget {
    pub fn content(&self) -> &str {
        self.0.content()
    }
}

impl<T> From<T> for SVGTarget
where
    T: Into<SVGDocument>,
{
    fn from(item: T) -> Self {
        SVGTarget(item.into())
    }
}

impl TessellationTarget for SVGTarget {
    fn get_data(
        &self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationProfile, Box<dyn std::error::Error>> {
        t.init(&self);

        Ok(t.tessellate()?)
    }

    fn time(
        &mut self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationTimeResult, Box<dyn std::error::Error>> {
        // Time pre-processing
        let t1 = Instant::now();
        t.init(&self);
        let t2 = Instant::now();
        let dur1 = t2.duration_since(t1);

        // Time the tessellation
        let t1 = Instant::now();
        t.tessellate()?;
        let t2 = Instant::now();
        let dur2 = t2.duration_since(t1);

        // Return duration passed
        Ok(TessellationTimeResult {
            init_time: dur1,
            tess_time: dur2,
        })
    }
}

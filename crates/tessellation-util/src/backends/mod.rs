mod lyon_tessellator;
use crate::artifacts::{PathProfile, TessellationData, TessellationProfile};
pub use lyon_tessellator::LyonTessellator;
use renderer::targets::SVGDocument;

pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGDocument);
    fn get_tessellation_profile(
        &self,
    ) -> Result<TessellationProfile, Box<dyn std::error::Error + Send + Sync>>;
    fn get_tessellation_path_profile(
        &self,
    ) -> Result<PathProfile, Box<dyn std::error::Error + Send + Sync>>;
    fn get_tessellation_data(
        &self,
    ) -> Result<TessellationData, Box<dyn std::error::Error + Send + Sync>>;
}
impl std::fmt::Debug for dyn Tessellator {
    fn fmt(
        &self,
        fmt: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(fmt, "Tessellator {{ name: {} }}", self.name())
    }
}

pub fn default() -> Box<dyn Tessellator> {
    Box::new(LyonTessellator::new())
}

pub fn all() -> Vec<Box<dyn Tessellator>> {
    let mut tessellators: Vec<Box<dyn Tessellator>> = vec![];
    tessellators.push(Box::new(LyonTessellator::new()));
    tessellators
}

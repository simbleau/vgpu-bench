mod lyon_tessellator;

use crate::{
    artifacts::{TessellationData, TessellationProfile},
    targets::SVGTarget,
};
pub use lyon_tessellator::LyonTessellator;

pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGTarget);
    fn tessellate(&self) -> Result<TessellationProfile, Box<dyn std::error::Error>>;
    fn get_tessellate_data(&self) -> Result<Box<TessellationData>, Box<dyn std::error::Error>>;
}
impl std::fmt::Debug for dyn Tessellator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
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

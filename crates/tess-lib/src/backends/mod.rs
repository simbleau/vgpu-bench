mod lyon_tessellator;
use crate::{
    artifacts::{TessellationData, TessellationProfile},
    targets::SVGDocument,
};
pub use lyon_tessellator::LyonTessellator;

pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGDocument);
    fn tessellate(&mut self) -> Result<TessellationProfile, Box<dyn std::error::Error>>;
    fn get_tessellate_data(&mut self) -> Result<Box<TessellationData>, Box<dyn std::error::Error>>;
}

pub fn all() -> Vec<Box<dyn Tessellator>> {
    let mut tessellators: Vec<Box<dyn Tessellator>> = vec![];
    tessellators.push(Box::new(LyonTessellator::new()));
    tessellators
}

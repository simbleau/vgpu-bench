use crate::{
    artifacts::{TessellationData, TessellationProfile},
    targets::SVGDocument,
};
use std::error::Error;
pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGDocument);
    fn tessellate(&mut self) -> Result<TessellationProfile, Box<dyn Error>>;
    fn get_tessellate_data(&mut self) -> Result<Box<TessellationData>, Box<dyn Error>>;
}

use crate::targets::{SVGDocument, TessellationProfile};
use std::error::Error;
pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGDocument);
    fn tessellate(&mut self) -> Result<TessellationProfile, Box<dyn Error>>;
}

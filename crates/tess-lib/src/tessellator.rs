use crate::targets::SVGDocument;
use std::error::Error;
pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGDocument);
    fn tessellate(&mut self) -> Result<(i32, i32), Box<dyn Error>>;
}

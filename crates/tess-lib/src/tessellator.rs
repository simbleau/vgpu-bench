use crate::TessellationTarget;
use std::error::Error;
pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &TessellationTarget);
    fn tessellate(&mut self) -> Result<(i32, i32), Box<dyn Error>>;
}

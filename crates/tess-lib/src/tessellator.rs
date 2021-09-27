use crate::targets::SVGDocument;
use std::error::Error;
use std::time::Duration;

pub struct TessellationProfileResult {
    pub vertices: i32,
    pub indices: i32,
}

pub struct TessellationTimeResult {
    pub init_time: Duration,
    pub tess_time: Duration,
}
pub trait Tessellator {
    fn name(&self) -> &'static str;
    fn init(&mut self, t: &SVGDocument);
    fn tessellate(&mut self) -> Result<TessellationProfileResult, Box<dyn Error>>;
}

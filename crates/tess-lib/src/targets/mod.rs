mod svg_document;
mod svg_file;

use std::time::Duration;

pub use svg_document::SVGDocument;
pub use svg_file::SVGFile;

use crate::Tessellator;

pub trait TessellationTarget {
    fn get_data(&self, t: Box<&mut dyn Tessellator>) -> (i32, i32);
    fn time(&mut self, t: Box<&mut dyn Tessellator>) -> (Duration, Duration);
}

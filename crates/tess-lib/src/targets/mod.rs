mod svg_document;
mod svg_file;

pub use svg_document::SVGDocument;
pub use svg_file::SVGFile;

use crate::{
    tessellator::{TessellationProfileResult, TessellationTimeResult},
    Tessellator,
};

pub trait TessellationTarget {
    fn get_data(&self, t: Box<&mut dyn Tessellator>) -> TessellationProfileResult;
    fn time(&mut self, t: Box<&mut dyn Tessellator>) -> TessellationTimeResult;
}

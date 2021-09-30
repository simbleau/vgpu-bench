use std::{borrow::Borrow, path::PathBuf};

use super::{SVGDocument, TessellationProfile, TessellationTarget, TessellationTimeResult};
use crate::Tessellator;
pub struct SVGFile {
    pub path: PathBuf,
}

impl From<&PathBuf> for SVGFile {
    fn from(item: &PathBuf) -> Self {
        SVGFile {
            path: item.to_path_buf(),
        }
    }
}

impl TessellationTarget for SVGFile {
    fn get_data(&self, t: Box<&mut dyn Tessellator>) -> TessellationProfile {
        let svg_document: SVGDocument = SVGDocument::from(self);
        svg_document.get_data(t)
    }

    fn time(&mut self, t: Box<&mut dyn Tessellator>) -> TessellationTimeResult {
        let file_ref: &SVGFile = self.borrow();
        let mut svg_document: SVGDocument = SVGDocument::from(file_ref);
        svg_document.time(t)
    }

    fn time_render(
        &mut self,
        t: Box<&mut dyn Tessellator>,
        frames: usize,
    ) -> crate::artifacts::FlatRenderTimeResult {
        let file_ref: &SVGFile = self.borrow();
        let mut svg_document: SVGDocument = SVGDocument::from(file_ref);
        svg_document.time_render(t, frames)
    }
}

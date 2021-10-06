use crate::artifacts::RenderTimeResult;
use crate::backends::Tessellator;
use crate::renderer;
use crate::targets::{
    SVGDocument, TessellationProfile, TessellationTarget, TessellationTimeResult,
};
use std::{borrow::Borrow, path::PathBuf};

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
    fn get_data(
        &self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationProfile, Box<dyn std::error::Error>> {
        let svg_document: SVGDocument = SVGDocument::from(self);
        svg_document.get_data(t)
    }

    fn time(
        &mut self,
        t: &mut dyn Tessellator,
    ) -> Result<TessellationTimeResult, Box<dyn std::error::Error>> {
        let file_ref: &SVGFile = self.borrow();
        let mut svg_document: SVGDocument = SVGDocument::from(file_ref);
        svg_document.time(t)
    }

    fn time_render(
        &mut self,
        t: &mut dyn Tessellator,
        frames: usize,
    ) -> renderer::error::Result<RenderTimeResult> {
        let file_ref: &SVGFile = self.borrow();
        let mut svg_document: SVGDocument = SVGDocument::from(file_ref);
        svg_document.time_render(t, frames)
    }
}

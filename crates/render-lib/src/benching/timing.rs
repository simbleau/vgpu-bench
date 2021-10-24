use crate::{artifacts::RenderTimeResult, rendering::Renderer};
use std::error::Error;
use tess_lib::targets::SVGDocument;

pub fn time_svg(
    renderer: &mut dyn Renderer,
    svg: &mut SVGDocument,
    frames: usize,
) -> Result<RenderTimeResult, Box<dyn Error>> {
    renderer.init()?;
    renderer.stage(svg)?;
    Ok(renderer.render(frames)?)
}

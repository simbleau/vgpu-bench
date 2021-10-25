use std::error::Error;

use renderer::{artifacts::RenderTimeResult, targets::SVGDocument, Renderer};

pub fn time_svg(
    renderer: &mut dyn Renderer,
    svg: &mut SVGDocument,
    frames: u64,
) -> Result<RenderTimeResult, Box<dyn Error>> {
    renderer.init()?;
    renderer.stage(svg)?;
    Ok(renderer.render(frames)?)
}

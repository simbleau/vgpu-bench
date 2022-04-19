use super::error::Result;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::SVGDocument;
use renderer::Renderer;

pub fn time_svg(
    renderer: &mut dyn Renderer,
    svg: &SVGDocument,
    frames: usize,
) -> Result<RenderTimeResult> {
    renderer.init()?;
    renderer.stage(svg)?;
    Ok(renderer.render(frames)?)
}

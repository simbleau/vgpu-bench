use super::error::Result;
use super::output::NaivePrimitiveRenderTime;
use super::timing;
use crate::benching::output::NaiveSVGFileRenderTime;
use naive_renderer::NaiveRenderer;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::{SVGDocument, SVGFile};
use renderer::Renderer;
use std::path::PathBuf;
use svg_generator::Primitive;
use tessellation_util::backends::Tessellator;

pub fn time_svg(
    renderer: &mut dyn Renderer,
    svg: &SVGDocument,
    frames: usize,
) -> Result<RenderTimeResult> {
    renderer.init()?;
    renderer.stage(svg)?;
    Ok(renderer.render(frames)?)
}

pub fn time_primitive(
    renderer: &mut dyn Renderer,
    primitive: Primitive,
    primitive_count: u32,
    frames: usize,
) -> Result<RenderTimeResult> {
    let svg_src = svg_generator::generate_svg(primitive, primitive_count, true);
    let svg = SVGDocument::from(svg_src);
    Ok(time_svg(renderer, &svg, frames)?)
}

pub fn time_naive_svg<P>(
    backend: &mut dyn Tessellator,
    svg_path: P,
    frames: usize,
) -> Result<Vec<NaiveSVGFileRenderTime>>
where
    P: Into<PathBuf>,
{
    let mut renderer = NaiveRenderer::new();

    let svg_path: PathBuf = svg_path.into();
    let svg_file = SVGFile::from(&svg_path);
    let svg = SVGDocument::try_from(svg_file)?;

    backend.init(&svg);
    let profile = backend.get_tessellation_profile()?;
    let render_time_result = timing::time_svg(&mut renderer, &svg, frames)?;

    let mut results: Vec<NaiveSVGFileRenderTime> = Vec::new();
    for (frame, dur) in render_time_result.frame_times.iter().enumerate() {
        let naive_rendertime = NaiveSVGFileRenderTime {
            tessellator: backend.name().to_owned(),
            filename: svg_path.display().to_string(),
            triangles: profile.triangles,
            frame: (frame + 1) as u32,
            frame_time: dur.as_nanos(),
        };
        results.push(naive_rendertime);
    }

    Ok(results)
}

pub fn time_naive_primitive(
    backend: &mut dyn Tessellator,
    primitive: Primitive,
    primitive_count: u32,
    frames: usize,
) -> Result<Vec<NaivePrimitiveRenderTime>> {
    let mut renderer = NaiveRenderer::new();

    let svg_src = svg_generator::generate_svg(primitive, primitive_count, true);
    let svg = SVGDocument::from(svg_src);

    backend.init(&svg);
    let profile = backend.get_tessellation_profile()?;
    let render_time_result = timing::time_svg(&mut renderer, &svg, frames)?;

    let mut results: Vec<NaivePrimitiveRenderTime> = Vec::new();
    for (frame, dur) in render_time_result.frame_times.iter().enumerate() {
        let naive_rendertime = NaivePrimitiveRenderTime {
            tessellator: backend.name().to_owned(),
            primitive: primitive.name().to_owned(),
            amount: primitive_count,
            triangles: profile.triangles,
            frame: (frame + 1) as u32,
            frame_time: dur.as_nanos(),
        };
        results.push(naive_rendertime);
    }

    Ok(results)
}

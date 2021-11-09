use super::error::Result;
use super::output::PrimitiveNaiveRenderTime;
use super::timing;
use crate::benching::output::SVGNaiveRenderTime;
use naive_renderer::NaiveRenderer;
use renderer::artifacts::RenderTimeResult;
use renderer::rust::Renderer;
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;
use svg_gen::Primitive;
use tess_lib::backends::Tessellator;
use tess_lib::targets::SVGTarget;

pub fn time_svg(
    renderer: &mut dyn Renderer,
    svg: &mut SVGDocument,
    frames: usize,
) -> Result<RenderTimeResult> {
    renderer.init()?;
    renderer.stage(svg)?;
    Ok(renderer.render(frames)?)
}

pub fn time_naive_svg<P>(
    backend: &mut dyn Tessellator,
    svg_path: P,
    frames: usize,
) -> Result<Vec<SVGNaiveRenderTime>>
where
    P: Into<PathBuf>,
{
    let mut renderer = NaiveRenderer::new();

    let svg_path: PathBuf = svg_path.into();
    let svg_file = SVGFile::from(&svg_path);
    let mut svg_doc = SVGDocument::from(svg_file);
    let svg_target = SVGTarget::from(svg_doc.clone());

    backend.init(&svg_target);
    let profile = backend.tessellate()?;
    let render_time_result = timing::time_svg(&mut renderer, &mut svg_doc, frames)?;

    let mut results: Vec<SVGNaiveRenderTime> = Vec::new();
    for (frame, dur) in render_time_result.frame_times.iter().enumerate() {
        let naive_rendertime = SVGNaiveRenderTime {
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

pub fn time_naive_primitive<P>(
    backend: &mut dyn Tessellator,
    primitive: Primitive,
    primitive_name: String,
    primitive_count: u32,
    frames: usize,
) -> Result<Vec<PrimitiveNaiveRenderTime>>
where
    P: Into<PathBuf>,
{
    let mut renderer = NaiveRenderer::new();

    let mut svg_doc = SVGDocument::from(svg_gen::generate_svg(primitive, primitive_count, true));
    let svg_target: SVGTarget = SVGTarget::from(svg_doc.clone());

    backend.init(&svg_target);
    let profile = backend.tessellate()?;
    let render_time_result = timing::time_svg(&mut renderer, &mut svg_doc, frames)?;

    let mut results: Vec<PrimitiveNaiveRenderTime> = Vec::new();
    for (frame, dur) in render_time_result.frame_times.iter().enumerate() {
        let naive_rendertime = PrimitiveNaiveRenderTime {
            tessellator: backend.name().to_owned(),
            primitive: primitive_name.clone(),
            amount: primitive_count,
            triangles: profile.triangles,
            frame: (frame + 1) as u32,
            frame_time: dur.as_nanos(),
        };
        results.push(naive_rendertime);
    }

    Ok(results)
}

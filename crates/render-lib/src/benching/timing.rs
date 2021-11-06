use super::error::{BenchingError::Logic, Result};
use super::output::PrimitiveFlatRenderTime;
use super::timing;
use crate::benching::output::SVGFlatRenderTime;
use renderer::artifacts::RenderTimeResult;
use renderer::targets::{SVGDocument, SVGFile};
use renderer::Renderer;
use std::{fs::File, path::PathBuf};
use svg_gen::Primitive;
use tess_lib::targets::{SVGTarget, TessellationTarget};
use tess_lib::{backends, backends::Tessellator};

pub fn write_flat_frametimes_svgs<P>(
    renderer: &mut dyn Renderer,
    svg_dir: P,
    output: P,
    frames: usize,
) -> Result<()>
where
    P: Into<PathBuf>,
{
    let files = super::io::get_files(svg_dir, false)?;
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, retrieve the file profiles
    for mut backend in backends::all() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow

        // Retrieve the profile from files and record the results
        for file in &files {
            let svg_file = SVGFile::from(file);
            let mut svg_doc = SVGDocument::from(svg_file);
            let svg_target = SVGTarget::from(svg_doc.clone());

            backend.init(&svg_target);
            let profile = backend.tessellate()?;
            let result = timing::time_svg(renderer, &mut svg_doc, frames)?;

            let filename = file
                .file_name()
                .ok_or(Logic("File name unkown"))?
                .to_string_lossy()
                .to_string();

            for frame in 0..result.frame_times.len() {
                let frame_time = result.frame_times[frame].as_nanos();
                let csv_entry = SVGFlatRenderTime {
                    tessellator: backend.name().to_owned(),
                    filename: filename.to_owned(),
                    triangles: profile.triangles,
                    frame: (frame + 1) as u32,
                    frame_time,
                };
                csv_wtr.serialize(csv_entry)?;
            }
        }
    }
    csv_wtr.flush()?;

    Ok(())
}

pub fn write_flat_frametimes_primitives<P>(
    renderer: &mut dyn Renderer,
    primitives: &Vec<(String, Primitive)>,
    count: u32,
    output: P,
    frames: usize,
) -> Result<()>
where
    P: Into<PathBuf>,
{
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, tessellate the files
    for mut backend in backends::all() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow
        for (prim_name, primitive) in primitives {
            let mut svg_doc = SVGDocument::from(svg_gen::generate_svg(*primitive, count, true));
            let target: SVGTarget = SVGTarget::from(svg_doc.clone());

            let profile = target.get_data(backend)?;
            let result = timing::time_svg(renderer, &mut svg_doc, frames)?;

            for frame in 0..result.frame_times.len() {
                let frame_time = result.frame_times[frame].as_nanos();
                let result = PrimitiveFlatRenderTime {
                    tessellator: backend.name().to_owned(),
                    primitive: prim_name.to_owned(),
                    amount: count,
                    triangles: profile.triangles,
                    frame: (frame + 1) as u32,
                    frame_time,
                };
                csv_wtr.serialize(result)?;
            }
        }
    }

    csv_wtr.flush()?;

    Ok(())
}

pub fn time_svg(
    renderer: &mut dyn Renderer,
    svg: &mut SVGDocument,
    frames: usize,
) -> Result<RenderTimeResult> {
    renderer.init()?;
    renderer.stage(svg)?;
    Ok(renderer.render(frames)?)
}

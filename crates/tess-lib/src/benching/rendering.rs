use svg_gen::Primitive;

use super::error::{BenchingError::Logic, Result};
use super::output::PrimitiveFlatRenderTime;
use crate::backends::Tessellator;
use crate::benching::output::SVGFlatRenderTime;
use crate::targets::{SVGDocument, SVGFile, TessellationTarget};
use std::{fs::File, path::PathBuf};

pub fn render_svgs<P>(svg_dir: P, output: P, frames: usize) -> Result<()>
where
    P: Into<PathBuf>,
{
    let files = super::io::get_files(svg_dir, false)?;
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, retrieve the file profiles
    for mut backend in crate::backends::all() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow

        // Retrieve the profile from files and record the results
        for file in &files {
            let mut target: SVGFile = file.into();
            let result = target.time_render(backend, frames)?;

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
                    triangles: result.triangles,
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

pub fn render_primitives<P>(
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
    for mut backend in crate::backends::all() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow
        for (prim_name, primitive) in primitives {
            let mut target = SVGDocument::from(svg_gen::generate_svg(*primitive, count, true));

            let profile = target.get_data(backend)?;
            let result = target.time_render(backend, frames)?;

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

use super::error::{BenchingError::Logic, Result};
use crate::backends::Tessellator;
use crate::benching::output::SVGFlatRenderTime;
use crate::targets::{SVGFile, TessellationTarget};
use std::{fs::File, path::PathBuf};

pub fn render_svgs<P>(svg_dir: P, output: P) -> Result<()>
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
            let result = target.time_render(backend, 5)?;

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

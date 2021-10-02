use std::{fs::File, path::PathBuf};

use crate::artifacts::serializable::SVGFlatRenderTime;
use crate::benching::error::BenchingError::Logic;
use crate::targets::TessellationTarget;
use crate::{targets::SVGFile, Tessellator};

use super::Result;

pub fn render_svgs<P>(svg_dir: P, output: P) -> Result<()>
where
    P: Into<PathBuf>,
{
    let files = super::get_files(svg_dir, false)?;
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, retrieve the file profiles
    for mut backend in crate::backends::backends() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow

        // Retrieve the profile from files and record the results
        for file in &files {
            let mut target: SVGFile = file.into();
            let result = target.time_render(Box::new(backend), 5);

            let filename = file
                .file_name()
                .ok_or(Logic("File name unkown"))?
                .to_string_lossy()
                .to_string();

            for frame_time in result.frame_times {
                let csv_entry = SVGFlatRenderTime {
                    tessellator: backend.name().to_owned(),
                    filename: filename.to_owned(),
                    triangles: result.triangles,
                    frame_time: frame_time.as_nanos(),
                };
                csv_wtr.serialize(csv_entry)?;
            }
        }
    }
    csv_wtr.flush()?;

    Ok(())
}

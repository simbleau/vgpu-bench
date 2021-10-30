use renderer::targets::{SVGDocument, SVGFile};

use super::error::{BenchingError::Logic, Result};
use crate::{
    backends::Tessellator,
    benching::output::SVGProfile,
    targets::{SVGTarget, TessellationTarget},
};
use std::{fs::File, path::PathBuf};

pub fn write_svg_profiles<P>(svg_dir: P, output: P) -> Result<()>
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
            let svg_file = SVGFile::from(file);
            let svg_doc = SVGDocument::from(svg_file);
            let target = SVGTarget::from(svg_doc);

            let profile_result = target.get_data(backend)?;

            let filename = file
                .file_name()
                .ok_or(Logic("File name unkown"))?
                .to_string_lossy()
                .to_string();
            let result = SVGProfile {
                tessellator: backend.name().to_owned(),
                filename,
                vertices: profile_result.vertices,
                indices: profile_result.indices,
            };
            csv_wtr.serialize(result)?;
        }
    }
    csv_wtr.flush()?;

    Ok(())
}

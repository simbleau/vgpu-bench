use std::{fs::File, path::PathBuf};

use crate::{
    artifacts::SVGProfileResult,
    targets::{SVGFile, TessellationTarget},
    Tessellator,
};

use super::Result;

pub fn profile_svgs<P>(svg_dir: P, output: P) -> Result<()>
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
            let target: SVGFile = file.into();
            let (vertices, indices) = target.get_data(Box::new(backend));

            let result = SVGProfileResult {
                tessellator: backend.name().to_owned(),
                filename: file.file_name().unwrap().to_str().unwrap().to_owned(),
                vertices,
                indices,
            };
            csv_wtr.serialize(result)?;
        }
    }
    csv_wtr.flush()?;

    Ok(())
}

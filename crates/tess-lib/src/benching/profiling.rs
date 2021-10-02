use std::{fs::File, path::PathBuf};

use crate::{
    artifacts::serializable::SVGProfile,
    targets::{SVGFile, TessellationTarget},
    Tessellator,
};

use crate::benching::error::BenchingError::Logic;

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
            let profile_result = target.get_data(Box::new(backend));

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

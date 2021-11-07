use renderer::targets::{SVGDocument, SVGFile};

use super::error::{BenchingError::Logic, Result};
use crate::{
    backends::Tessellator,
    benching::output::SVGProfile,
    targets::{SVGTarget, TessellationTarget},
};
use std::{fs::File, path::PathBuf};

pub fn get_profile<P: Into<PathBuf>>(
    backend: &mut dyn Tessellator,
    file_path: P,
) -> Result<SVGProfile> {
    let path: PathBuf = file_path.into();
    let svg_file = SVGFile::from(&path);
    let svg_doc = SVGDocument::from(svg_file);
    let target = SVGTarget::from(svg_doc);

    let tessellator_name = backend.name().to_owned();
    let filename = path
        .file_name()
        .ok_or(Logic("File name unkown"))?
        .to_string_lossy()
        .to_string();

    let profile_result = target.get_data(backend)?;

    Ok(SVGProfile {
        tessellator: tessellator_name,
        filename,
        vertices: profile_result.vertices,
        indices: profile_result.indices,
        triangles: profile_result.triangles,
    })
}

pub fn get_profiles<P: Into<PathBuf>>(
    mut backend: Box<dyn Tessellator>,
    dir_path: P,
) -> Result<Vec<SVGProfile>> {
    let files = super::io::get_files(dir_path, false)?;
    let results: Result<Vec<SVGProfile>> = files
        .iter()
        .map(|file| get_profile(backend.as_mut(), file))
        .collect();
    Ok(results?)
}

pub fn write_svg_profiles<P>(svg_dir: P, output: P) -> Result<()>
where
    P: Into<PathBuf>,
{
    /*
    let files = super::io::get_files(svg_dir, false)?;
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, retrieve the file profiles
    let backends = crate::backends::all();
    for backend in backends {
        let tessellator_name = backend.name().to_owned();

        // Retrieve the profile from files and record the results
        for file in &files {
            let svg_file = SVGFile::from(file);
            let svg_doc = SVGDocument::from(svg_file);
            let target = SVGTarget::from(svg_doc);

            let filename = file
                .file_name()
                .ok_or(Logic("File name unkown"))?
                .to_string_lossy()
                .to_string();
            let profile_result = target.get_data(backend.as_mut())?;

            let result = SVGProfile {
                tessellator: tessellator_name.to_owned(),
                filename,
                vertices: profile_result.vertices,
                indices: profile_result.indices,
                triangles: profile_result.triangles,
            };
            csv_wtr.serialize(result)?;
        }
    }
    csv_wtr.flush()?;

    Ok(())
    */
    Ok(())
}

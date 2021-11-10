use crate::{backends::Tessellator, benching::error::Result, benching::output::SVGProfile};
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;

pub fn get_profile<P: Into<PathBuf>>(
    backend: &mut dyn Tessellator,
    file_path: P,
) -> Result<SVGProfile> {
    let path: PathBuf = file_path.into();
    let svg_file = SVGFile::from(&path);
    let svg_doc = SVGDocument::from(svg_file);

    backend.init(&svg_doc);
    let profile = backend.get_tessellation_profile()?;

    Ok(SVGProfile {
        tessellator: backend.name().to_owned(),
        filename: path.display().to_string(),
        vertices: profile.vertices,
        indices: profile.indices,
        triangles: profile.triangles,
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

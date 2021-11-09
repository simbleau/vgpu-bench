use crate::{
    backends::Tessellator,
    benching::error::Result,
    benching::output::SVGProfile,
    targets::{SVGTarget, TessellationTarget},
};
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;

pub fn get_profile<P: Into<PathBuf>>(
    backend: &mut dyn Tessellator,
    file_path: P,
) -> Result<SVGProfile> {
    let path: PathBuf = file_path.into();
    let svg_file = SVGFile::from(&path);
    let svg_doc = SVGDocument::from(svg_file);
    let target = SVGTarget::from(svg_doc);

    let profile_result = target.get_data(backend)?;

    Ok(SVGProfile {
        tessellator: backend.name().to_owned(),
        filename: path.display().to_string(),
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

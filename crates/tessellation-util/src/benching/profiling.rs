use crate::{
    backends::Tessellator, benching::error::Result,
    benching::output::SVGFileProfile,
};
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;
use svg_generator::Primitive;

use super::output::SVGPrimitiveProfile;

pub fn get_file_profile<P: Into<PathBuf>>(
    backend: &mut dyn Tessellator,
    file_path: P,
) -> Result<SVGFileProfile> {
    let path: PathBuf = file_path.into();
    let svg_file = SVGFile::from(&path);
    let svg_doc = SVGDocument::try_from(svg_file)?;

    backend.init(&svg_doc);
    let profile = backend.get_tessellation_profile()?;

    Ok(SVGFileProfile {
        tessellator: backend.name().to_owned(),
        filename: path.display().to_string(),
        vertices: profile.vertices,
        indices: profile.indices,
        triangles: profile.triangles,
    })
}

pub fn get_primitive_profile(
    backend: &mut dyn Tessellator,
    primitive: Primitive,
    primitive_count: u32,
) -> Result<SVGPrimitiveProfile> {
    let svg_src = svg_generator::generate_svg(primitive, primitive_count, true);
    let svg = SVGDocument::from(svg_src);

    backend.init(&svg);
    let profile = backend.get_tessellation_profile()?;

    Ok(SVGPrimitiveProfile {
        tessellator: backend.name().to_owned(),
        primitive: primitive.name().to_owned(),
        primitive_count,
        vertices: profile.vertices,
        indices: profile.indices,
        triangles: profile.triangles,
    })
}

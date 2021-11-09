use std::path::PathBuf;

use renderer::targets::{SVGDocument, SVGFile};
use svg_gen::Primitive;

use crate::targets::TessellationTarget;
use crate::{backends::Tessellator, targets::SVGTarget};

use super::error::Result;
use super::output::{
    PrimitiveTessellationTime, SVGDocumentTessellationTime, SVGFileTessellationTime,
};

pub fn time_svg_file<P: Into<PathBuf>>(
    backend: &mut dyn Tessellator,
    file_path: P,
    trials: u32,
) -> Result<Vec<SVGFileTessellationTime>> {
    let file_path: PathBuf = file_path.into();
    let svg_file = SVGFile::from(&file_path.clone().into());
    let mut svg_doc = SVGDocument::from(svg_file);

    Ok(time_svg_doc(backend, &mut svg_doc, trials)?
        .into_iter()
        .map(|x| SVGFileTessellationTime {
            tessellator: x.tessellator,
            filename: file_path.display().to_string(),
            init_time: x.init_time,
            tess_time: x.tess_time,
        })
        .collect())
}

pub fn time_svg_doc(
    backend: &mut dyn Tessellator,
    svg_doc: &mut SVGDocument,
    trials: u32,
) -> Result<Vec<SVGDocumentTessellationTime>> {
    let mut target = SVGTarget::from(svg_doc.clone());
    let mut results: Vec<SVGDocumentTessellationTime> = Vec::new();

    for _ in 0..trials {
        let time_result = target.time(backend)?;

        let result = SVGDocumentTessellationTime {
            tessellator: backend.name().to_owned(),
            init_time: time_result.init_time.as_nanos(),
            tess_time: time_result.tess_time.as_nanos(),
        };
        results.push(result);
    }
    Ok(results)
}

pub fn time_primitive(
    backend: &mut dyn Tessellator,
    primitive: Primitive,
    primitive_count: u32,
    trials: u32,
) -> Result<Vec<PrimitiveTessellationTime>> {
    let svg_src = svg_gen::generate_svg(primitive, primitive_count, true);
    let svg_doc = SVGDocument::from(svg_src);
    let mut target = SVGTarget::from(svg_doc);

    let mut results: Vec<PrimitiveTessellationTime> = Vec::new();
    for _ in 0..trials {
        let time_result = target.time(backend)?;

        let result = PrimitiveTessellationTime {
            tessellator: backend.name().to_owned(),
            primitive: primitive.name().to_owned(),
            amount: primitive_count,
            init_time: time_result.init_time.as_nanos(),
            tess_time: time_result.tess_time.as_nanos(),
        };
        results.push(result);
    }
    Ok(results)
}

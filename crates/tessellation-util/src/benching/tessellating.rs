use super::error::Result;
use super::output::{
    PrimitiveTessellationTime, SVGDocumentTessellationTime,
    SVGFileTessellationTime,
};
use crate::backends::Tessellator;
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;
use std::time::Instant;
use svg_gen::Primitive;

pub fn time_svg_file<P: Into<PathBuf>>(
    backend: Box<dyn Tessellator>,
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
    mut backend: Box<dyn Tessellator>,
    svg: &SVGDocument,
    trials: u32,
) -> Result<Vec<SVGDocumentTessellationTime>> {
    let mut results: Vec<SVGDocumentTessellationTime> = Vec::new();

    for _ in 0..trials {
        // Time initialization
        let t1 = Instant::now();
        backend.init(svg);
        let t2 = Instant::now();
        let dur1 = t2.duration_since(t1);

        // Time the tessellation
        let t1 = Instant::now();
        backend.get_tessellation_profile()?;
        let t2 = Instant::now();
        let dur2 = t2.duration_since(t1);

        let result = SVGDocumentTessellationTime {
            tessellator: backend.name().to_owned(),
            init_time: dur1.as_nanos(),
            tess_time: dur2.as_nanos(),
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
    let svg = SVGDocument::from(svg_src);

    let mut results: Vec<PrimitiveTessellationTime> = Vec::new();
    for _ in 0..trials {
        // Time initialization
        let t1 = Instant::now();
        backend.init(&svg);
        let t2 = Instant::now();
        let dur1 = t2.duration_since(t1);

        // Time the tessellation
        let t1 = Instant::now();
        backend.get_tessellation_profile()?;
        let t2 = Instant::now();
        let dur2 = t2.duration_since(t1);

        let result = PrimitiveTessellationTime {
            tessellator: backend.name().to_owned(),
            primitive: primitive.name().to_owned(),
            amount: primitive_count,
            init_time: dur1.as_nanos(),
            tess_time: dur2.as_nanos(),
        };
        results.push(result);
    }
    Ok(results)
}

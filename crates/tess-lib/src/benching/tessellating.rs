use renderer::targets::SVGDocument;
use svg_gen::Primitive;

use crate::targets::TessellationTarget;
use crate::{backends::Tessellator, targets::SVGTarget};

use super::error::Result;
use super::output::PrimitiveTessellationTime;

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

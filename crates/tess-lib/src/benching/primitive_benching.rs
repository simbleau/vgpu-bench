use std::{fs::File, path::PathBuf};

use svg_gen::Primitive;

use super::Result;
use crate::{
    artifacts::PrimitiveTimeResult,
    targets::{SVGDocument, TessellationTarget},
    Tessellator,
};

pub fn time_primitive<P>(
    prim_name: String,
    primitive: Primitive,
    output: P,
    trials: i32,
) -> Result<()>
where
    P: Into<PathBuf>,
{
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, tessellate the files
    for mut backend in crate::backends::backends() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow
        let counts = std::iter::once(1).chain((10..=50).step_by(10));
        for count in counts.clone() {
            for _ in 0..trials {
                let mut target: SVGDocument =
                    SVGDocument::from(svg_gen::generate_svg(primitive, count, true));
                let (init_time, tess_time) = target.time(Box::new(backend));

                let result = PrimitiveTimeResult {
                    tessellator: backend.name().to_owned(),
                    primitive: prim_name.to_owned(),
                    init_time: init_time.as_nanos() as i32,
                    tess_time: tess_time.as_nanos() as i32,
                };
                csv_wtr.serialize(result)?;
            }
        }
    }

    csv_wtr.flush()?;

    Ok(())
}

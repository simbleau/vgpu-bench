use super::error::Result;
use crate::{
    backends::Tessellator,
    benching::output::PrimitiveTime,
    targets::{SVGDocument, TessellationTarget},
};
use std::{fs::File, path::PathBuf};
use svg_gen::Primitive;

pub fn time_primitives<P>(
    primitives: &Vec<(String, Primitive)>,
    output: P,
    max_prims: u32,
    step_size: u32,
    trials: u32,
) -> Result<()>
where
    P: Into<PathBuf>,
{
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, tessellate the files
    for (prim_name, primitive) in primitives {
        for mut backend in crate::backends::all() {
            let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow
            let counts = (step_size..max_prims).step_by(step_size as usize);
            for count in counts.clone() {
                let mut target: SVGDocument =
                    SVGDocument::from(svg_gen::generate_svg(*primitive, count, true));
                for _ in 0..trials {
                    let time_result = target.time(backend)?;

                    let result = PrimitiveTime {
                        tessellator: backend.name().to_owned(),
                        prim_name: prim_name.to_owned(),
                        amount: count,
                        init_time: time_result.init_time.as_nanos(),
                        tess_time: time_result.tess_time.as_nanos(),
                    };
                    csv_wtr.serialize(result)?;
                }
            }
        }
    }

    csv_wtr.flush()?;

    Ok(())
}

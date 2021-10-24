use std::path::PathBuf;

use svg_gen::Primitive;

pub fn write_svg_profiles<P>(svg_dir: P, output: P) -> tess::benching::error::Result<()>
where
    P: Into<PathBuf>,
{
    // TODO : Surely I shouldn't be re-exporting an identical function?
    Ok(tess::benching::profiling::profile_svgs(svg_dir, output)?)
}

pub fn write_primitive_tessellation_times<P>(
    primitives: &Vec<(String, Primitive)>,
    max_prims: u32,
    step_size: u32,
    trials: u32,
    output: P,
) -> tess::benching::error::Result<()>
where
    P: Into<PathBuf>,
{
    // TODO : Surely I shouldn't be re-exporting an identical function?
    Ok(tess::benching::tessellating::time_primitives(
        primitives, output, max_prims, step_size, trials,
    )?)
}

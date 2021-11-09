use crate::dictionary::{EXAMPLES_ASSETS_DIR, EXAMPLES_OUTPUT_DIR};
use crate::dictionary::{PRIMITIVES_ASSETS_DIR, PRIMITIVES_OUTPUT_DIR};
use const_format::concatcp;
use log::{debug, error, info, trace};

pub fn profile_svg_examples() {
    let input_dir_path = EXAMPLES_ASSETS_DIR;
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];

    trace!("Commencing SVG example profiling");
    debug!(
        "SVG Profiling:
\tInput directory: '{input_dir_path}'
\tOutput path: '{output_path}'"
    );
    if let Err(x) =
        vgpu_bench::benchmarks::tessellation::profile_dir_with_output(input_dir_path, output_path)
    {
        error!("{:?}", x);
    }
    trace!("Completed SVG example profiling");
    info!("Completed SVG Profiling. Output to '{}'", output_path);
}

pub fn profile_svg_primitives() {
    let input_dir_path = PRIMITIVES_ASSETS_DIR;
    let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];

    trace!("Commencing SVG primitive profiling");
    debug!(
        "SVG Profiling:
\tInput directory: '{input_dir_path}'
\tOutput path: '{output_path}'"
    );
    if let Err(x) =
        vgpu_bench::benchmarks::tessellation::profile_dir_with_output(input_dir_path, output_path)
    {
        error!("{:?}", x);
    }
    trace!("Completed SVG primitive profiling");
    info!("Completed SVG Profiling. Output to '{}'", output_path);
}

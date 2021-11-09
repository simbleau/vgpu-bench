use crate::dictionary::{EXAMPLES_ASSETS_DIR, EXAMPLES_OUTPUT_DIR};
use crate::dictionary::{PRIMITIVES_ASSETS_DIR, PRIMITIVES_OUTPUT_DIR};
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::tessellation::profile::SVGProfilingOptions;
use vgpu_bench::{benchmarks, util};

pub fn profile_svg_examples() {
    let input_dir_path = EXAMPLES_ASSETS_DIR;
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let writer = util::csv_writer(output_path).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let options = SVGProfilingOptions::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend);
    debug!("Options: {:?}", options);

    trace!("Commencing SVG example profiling");
    match benchmarks::tessellation::profile::write_profiles(options) {
        Ok(_) => {
            trace!("Completed SVG example profiling");
            info!("Completed SVG Profiling. Output to '{}'", output_path);
        }
        Err(err) => error!("{:?}", err),
    }
}

pub fn profile_svg_primitives() {
    let input_dir_path = PRIMITIVES_ASSETS_DIR;
    let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let writer = util::csv_writer(output_path).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let options = SVGProfilingOptions::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend);
    debug!("Options: {:?}", options);

    trace!("Commencing SVG primitive profiling");
    match benchmarks::tessellation::profile::write_profiles(options) {
        Ok(_) => {
            trace!("Completed SVG primitive profiling");
            info!("Completed SVG Profiling. Output to '{}'", output_path);
        }
        Err(err) => error!("{:?}", err),
    }
}

use crate::dictionary::{EXAMPLES_ASSETS_DIR, EXAMPLES_OUTPUT_DIR};
use crate::dictionary::{PRIMITIVES_ASSETS_DIR, PRIMITIVES_OUTPUT_DIR};
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::tessellation::profile::SVGProfiler;
use vgpu_bench::{benchmarks, util};

pub fn profile_svg_examples() {
    let input_dir_path = EXAMPLES_ASSETS_DIR;
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let output_file = util::create_file(output_path).expect("Could not create output file");
    let writer = csv::Writer::from_writer(output_file);
    let backend = tessellation::backends::default();

    let num_input_files = input_files.len();
    let backend_name = backend.name();

    trace!("Commencing SVG example profiling");
    debug!(
        "SVG Profiling:
\tInput directory({num_input_files} files): '{input_dir_path}'
\tOutput path: '{output_path}'
\tBackend: '{backend_name}'"
    );
    // Make profiler with options
    let profiler = SVGProfiler::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend);
    match benchmarks::tessellation::profile::profile(profiler) {
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
    let output_file = util::create_file(output_path).expect("Could not create output file");
    let writer = csv::Writer::from_writer(output_file);
    let backend = tessellation::backends::default();

    let num_input_files = input_files.len();
    let backend_name = backend.name();

    trace!("Commencing SVG primitive profiling");
    debug!(
        "SVG Profiling:
\tInput directory({num_input_files} files): '{input_dir_path}'
\tOutput path: '{output_path}'
\tBackend: '{backend_name}'"
    );
    // Make profiler with options
    let profiler = SVGProfiler::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend);
    match benchmarks::tessellation::profile::profile(profiler) {
        Ok(_) => {
            trace!("Completed SVG primitive profiling");
            info!("Completed SVG Profiling. Output to '{}'", output_path);
        }
        Err(err) => error!("{:?}", err),
    }
}

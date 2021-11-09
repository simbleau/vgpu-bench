use crate::dictionary::{EXAMPLES_ASSETS_DIR, EXAMPLES_OUTPUT_DIR};
use crate::dictionary::{PRIMITIVES_ASSETS_DIR, PRIMITIVES_OUTPUT_DIR};
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::tessellation::primitive_timing::PrimitiveTessellationTimingOptions;
use vgpu_bench::benchmarks::tessellation::profile::SVGProfilingOptions;
use vgpu_bench::{benchmarks, util};

pub fn profile_svg_examples() {
    let input_dir_path = EXAMPLES_ASSETS_DIR;
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
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
            info!(
                "Completed SVG example profiling. Output to '{}'",
                output_path
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

pub fn profile_svg_primitives() {
    let input_dir_path = PRIMITIVES_ASSETS_DIR;
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];
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
            info!(
                "Completed SVG primitive profiling. Output to '{}'",
                output_path
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

pub fn bench_tessellation_primitives() {
    let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "tessellation.csv"];
    let writer = util::csv_writer(output_path).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let primitives = svg_generator::primitives();
    let trials = 100;
    let options = PrimitiveTessellationTimingOptions::new()
        .writer(writer)
        .backend(backend)
        .primitives(primitives)
        .primitive_count(1)
        .primitive_count(100)
        .primitives_counts((1000..=10000).step_by(1000 as usize))
        .trials(trials);
    debug!("Options: {:?}", options);

    trace!("Commencing SVG primitive tessellation time capture");
    match benchmarks::tessellation::primitive_timing::write_tessellation_times(options) {
        Ok(_) => {
            trace!("Completed SVG primitive tessellation time capture");
            info!(
                "Completed SVG primitive tessellation time capture. Output to '{}'",
                output_path
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

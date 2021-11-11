use crate::dictionary::*;
use crate::driver::DriverOptions;
use log::{debug, error, info, trace};
use std::path::PathBuf;
use vgpu_bench::benchmarks::tessellation::primitive_timing::PrimitiveTessellationTimingOptions;
use vgpu_bench::benchmarks::tessellation::profile::SVGProfilingOptions;
use vgpu_bench::{benchmarks, util};

pub fn profile_svg_files<P>(options: &DriverOptions, input_dir_path: P)
where
    P: Into<PathBuf>,
{
    trace!("Commencing SVG file profiling");

    let output_path = options.output_dir.join(
        [
            DATA_DIR_NAME,
            EXAMPLES_DIR_NAME,
            SVG_DIR_NAME,
            "profiles.csv",
        ]
        .iter()
        .collect::<PathBuf>(),
    );
    let input_files =
        util::get_files_with_extension(input_dir_path, false, "svg");
    let writer = util::csv_writer(output_path.to_owned())
        .expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let options = SVGProfilingOptions::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend);
    debug!("Options: {:?}", options);

    match benchmarks::tessellation::profile::write_profiles(options) {
        Ok(_) => {
            trace!("Completed SVG file profiling");
            info!(
                "Completed SVG file profiling. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

// TODO generate as Primitives -> SVGs instead of using a hardcoded (cached)
// directory
pub fn profile_svg_primitives(options: &DriverOptions) {
    trace!("Commencing SVG primitive profiling");

    let output_path = options.output_dir.join(
        [
            DATA_DIR_NAME,
            PRIMITIVES_DIR_NAME,
            SVG_DIR_NAME,
            "profiles.csv",
        ]
        .iter()
        .collect::<PathBuf>(),
    );
    let input_dir_path = [ASSETS_DIR_NAME, SVG_DIR_NAME, PRIMITIVES_DIR_NAME]
        .iter()
        .collect::<PathBuf>();
    let input_files =
        util::get_files_with_extension(input_dir_path, false, "svg");
    let writer = util::csv_writer(output_path.to_owned())
        .expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let options = SVGProfilingOptions::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend);
    debug!("Options: {:?}", options);

    match benchmarks::tessellation::profile::write_profiles(options) {
        Ok(_) => {
            trace!("Completed SVG primitive profiling");
            info!(
                "Completed SVG primitive profiling. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

pub fn bench_tessellation_primitives(options: &DriverOptions) {
    let output_path = options.output_dir.join(
        [
            DATA_DIR_NAME,
            PRIMITIVES_DIR_NAME,
            SVG_DIR_NAME,
            "tessellation.csv",
        ]
        .iter()
        .collect::<PathBuf>(),
    );
    let writer = util::csv_writer(output_path.to_owned())
        .expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let primitives = svg_generator::primitives::default();
    let trials = 1;
    let options = PrimitiveTessellationTimingOptions::new()
        .writer(writer)
        .backend(backend)
        .primitives(primitives)
        .primitive_count(10)
        .primitives_counts((100..=500).step_by(100 as usize))
        .trials(trials);
    debug!("Options: {:?}", options);

    trace!("Commencing SVG primitive tessellation time capture");
    match benchmarks::tessellation::primitive_timing::write_tessellation_times(
        options,
    ) {
        Ok(_) => {
            trace!("Completed SVG primitive tessellation time capture");
            info!(
                "Completed SVG primitive tessellation time capture. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

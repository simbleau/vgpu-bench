use std::path::{Path, PathBuf};

use crate::dictionary::*;
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::tessellation::primitive_timing::PrimitiveTessellationTimingOptions;
use vgpu_bench::benchmarks::tessellation::profile::SVGProfilingOptions;
use vgpu_bench::{benchmarks, util};

pub fn profile_svg_files<P>(output_dir: &Path, input_dir_path: P)
where
    P: Into<PathBuf>,
{
    trace!("Commencing SVG file profiling");

    let output_path = output_dir.join(concatcp![DATA, EXAMPLES, SVG, "profiles.csv"]);
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let writer = util::csv_writer(output_path.to_owned()).expect("Could not create output file");
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

// TODO generate as Primitives -> SVGs instead of using a hardcoded (cached) directory
pub fn profile_svg_primitives(output_dir: &Path) {
    trace!("Commencing SVG primitive profiling");

    let output_path = output_dir.join(concatcp![DATA, PRIMITIVES, SVG, "profiles.csv"]);
    let input_dir_path = concatcp![ASSETS_DIR, SVG, PRIMITIVES];
    let input_files = util::get_files_with_extension(input_dir_path, false, "svg");
    let writer = util::csv_writer(output_path.to_owned()).expect("Could not create output file");
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

pub fn bench_tessellation_primitives(output_dir: &Path) {
    let output_path = output_dir.join(concatcp![DATA, PRIMITIVES, SVG, "tessellation.csv"]);
    let writer = util::csv_writer(output_path.to_owned()).expect("Could not create output file");
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
    match benchmarks::tessellation::primitive_timing::write_tessellation_times(options) {
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

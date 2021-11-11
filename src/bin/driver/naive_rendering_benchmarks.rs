use std::path::{Path, PathBuf};

use crate::dictionary::*;
use const_format::concatcp;
use log::{debug, error, info, trace};
use vgpu_bench::benchmarks::rendering::naive_primitive_rendering::PrimitiveNaiveRenderingOptions;
use vgpu_bench::benchmarks::rendering::naive_svg_rendering::SVGNaiveRenderingOptions;
use vgpu_bench::{benchmarks, util};

pub fn frametimes_svg_files<P>(output_dir: &Path, input_dir: P)
where
    P: Into<PathBuf>,
{
    trace!("Commencing naive SVG file rendering for frametime capture");

    let output_path = output_dir.join(concatcp![DATA, EXAMPLES, SVG, "naive_frametimes.csv"]);
    let input_files = util::get_files_with_extension(input_dir, false, "svg");
    let writer = util::csv_writer(output_path.to_owned()).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let frames = 500;
    let options = SVGNaiveRenderingOptions::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend)
        .frames(frames);
    debug!("Options: {:?}", options);

    match benchmarks::rendering::naive_svg_rendering::write_frametimes(options) {
        Ok(_) => {
            trace!("Completed naive SVG file rendering for frametime capture");
            info!(
                "Completed naive SVG file rendering for frametime capture. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

pub fn frametimes_svg_primitives(output_dir: &Path) {
    trace!("Commencing naive SVG primitive rendering for frametime capture");

    let output_path = output_dir.join(concatcp![DATA, PRIMITIVES, SVG, "naive_frametimes.csv"]);
    let writer = util::csv_writer(output_path.to_owned()).expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let primitives = svg_generator::primitives::default();
    let primitive_count = 1;
    let frames = 500;
    let options = PrimitiveNaiveRenderingOptions::new()
        .writer(writer)
        .primitives(primitives)
        .primitive_count(primitive_count)
        .backend(backend)
        .frames(frames);
    debug!("Options: {:?}", options);

    match benchmarks::rendering::naive_primitive_rendering::write_frametimes(options) {
        Ok(_) => {
            trace!("Completed naive SVG primitive rendering for frametime capture");
            info!(
                "Completed naive SVG primitive rendering for frametime capture. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

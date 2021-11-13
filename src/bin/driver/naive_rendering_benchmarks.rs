use log::{debug, error, info, trace};
use std::path::PathBuf;
use vgpu_bench::benchmarks::rendering::naive_primitive_rendering::PrimitiveNaiveRenderingOptions;
use vgpu_bench::benchmarks::rendering::naive_svg_rendering::NaiveSVGRenderingBuilder;
use vgpu_bench::driver::{dictionary::*, DriverOptions};
use vgpu_bench::{benchmarks, util};

pub fn frametimes_svg_files<P>(options: &DriverOptions, input_dir: P)
where
    P: Into<PathBuf>,
{
    trace!("Commencing naive SVG file rendering for frametime capture");

    let output_path: PathBuf = options.output_dir.join(
        [
            DATA_DIR_NAME,
            EXAMPLES_DIR_NAME,
            SVG_DIR_NAME,
            "naive_frametimes.csv",
        ]
        .iter()
        .collect::<PathBuf>(),
    );
    let input_files = util::get_files_with_extension(input_dir, false, "svg");
    let writer = util::csv_writer(output_path.to_owned())
        .expect("Could not create output file");
    let backend = tessellation_util::backends::default();
    let frames = 500;
    let options = NaiveSVGRenderingBuilder::new()
        .writer(writer)
        .assets(input_files)
        .backend(backend)
        .frames(frames);
    debug!("Options: {:?}", options);

    match benchmarks::rendering::naive_svg_rendering::frametimes(options) {
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

pub fn frametimes_svg_primitives(options: &DriverOptions) {
    trace!("Commencing naive SVG primitive rendering for frametime capture");

    let output_path = options.output_dir.join(
        [
            DATA_DIR_NAME,
            PRIMITIVES_DIR_NAME,
            SVG_DIR_NAME,
            "naive_frametimes.csv",
        ]
        .iter()
        .collect::<PathBuf>(),
    );
    let writer = util::csv_writer(output_path.to_owned())
        .expect("Could not create output file");
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

    match benchmarks::rendering::naive_primitive_rendering::write_frametimes(
        options,
    ) {
        Ok(_) => {
            trace!(
                "Completed naive SVG primitive rendering for frametime capture"
            );
            info!(
                "Completed naive SVG primitive rendering for frametime capture. Output to '{}'",
                output_path.display()
            );
        }
        Err(err) => error!("{:?}", err),
    }
}

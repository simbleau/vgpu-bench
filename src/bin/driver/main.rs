#![feature(format_args_capture)]

mod naive_rendering_benchmarks;
mod tessellation_benchmarks;

use chrono::Local;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::PathBuf;
use vgpu_bench::{
    benchmarks::BenchmarkBuilder,
    driver::{dictionary::*, Driver},
    util::{self, create_file},
};

pub fn main() {
    let input_dir_path = [ASSETS_DIR_NAME, SVG_DIR_NAME, PRIMITIVES_DIR_NAME]
        .iter()
        .collect::<PathBuf>();
    let input_files =
        util::get_files_with_extension(input_dir_path, false, "svg");
    let x = vgpu_bench::benchmarks::rendering::naive_svg_rendering::NaiveSVGRenderingBuilder::new()
    .writer(util::csv_writer("test").expect("Could not create output file"))
    .frames(1)
    .backend(tessellation_util::backends::default())
    .asset(input_files.first().unwrap());

    let output_dir = PathBuf::from("output/")
        .join(Local::now().format("%d%mY_%H-%M-%S").to_string());

    Driver::builder()
        .output_dir(output_dir.as_path())
        .logger(TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .logger(WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            create_file(output_dir.join("trace.log")).unwrap(),
        ))
        .add(x.build())
        /*
        .add(|opts| {
            tessellation_benchmarks::profile_svg_files(
                opts,
                "assets/svg/examples/",
            )
        })
        .add(tessellation_benchmarks::profile_svg_primitives)
        .add(|opts| {
            naive_rendering_benchmarks::frametimes_svg_files(
                opts,
                "assets/svg/examples/",
            )
        })
        .add(naive_rendering_benchmarks::frametimes_svg_primitives)
        */
        .build()
        .run();
}

#![feature(format_args_capture)]

mod naive_rendering_benchmarks;
mod tessellation_benchmarks;

use chrono::Local;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::PathBuf;
use vgpu_bench::{
    benchmarks::rendering::TimeNaiveSVGFileRendering,
    driver::Driver,
    util::{self, create_file},
};

pub fn main() {
    let output_dir = PathBuf::from("output/")
        .join(Local::now().format("%d%m%Y_%H-%M-%S").to_string());

    Driver::builder()
        .output_dir(output_dir.as_path())
        .logger(TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .logger(WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            create_file(output_dir.join("trace.log")).unwrap(),
        ))
        .add(
            TimeNaiveSVGFileRendering::new()
                .to_file("naive_frametimes.csv")
                .frames(1)
                .backend(tessellation_util::backends::default())
                .assets(util::get_files("assets/svg/examples", false)),
        )
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

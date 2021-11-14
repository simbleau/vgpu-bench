#![feature(format_args_capture)]

mod tessellation_benchmarks;

use chrono::Local;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::PathBuf;
use svg_generator::Primitive;
use vgpu_bench::{
    benchmarks::{
        rendering::{
            TimeNaiveSVGFileRendering, TimeNaiveSVGPrimitiveRendering,
        },
        tessellation::{ProfileSVGFiles, ProfileSVGPrimitives},
    },
    driver::Driver,
    util::{self, create_file},
};

pub fn main() {
    let output_dir = PathBuf::from("output/")
        .join(Local::now().format("%d%m%Y_%H-%M-%S").to_string());

    Driver::builder()
        .on_error_panic()
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
        /*
        .add(
            TimeNaiveSVGFileRendering::new()
                .to_file("naive_file_frametimes")
                .frames(1)
                .backend(tessellation_util::backends::default())
                .assets(util::get_files("assets/svg/examples", false)),
        )
        .add(
            TimeNaiveSVGPrimitiveRendering::new()
                .to_file("naive_primitive_frametimes")
                .backend(tessellation_util::backends::default())
                .frames(1)
                .primitives(svg_generator::primitives::default())
                .primitive_count(1),
        )
        */
        .add(
            ProfileSVGFiles::new()
                .to_file("file_profiles")
                .to_plot("file_profiles")
                .backend(tessellation_util::backends::default())
                .assets(util::get_files("assets/svg/examples", false)),
        )
        /*
        .add(
            ProfileSVGPrimitives::new()
                .to_file("primitive_profiles")
                .backend(tessellation_util::backends::default())
                .primitives(svg_generator::primitives::default())
                .primitive_count(10)
                .primitives_counts((100..=500).step_by(100 as usize)),
        )
        */
        .build()
        .run();
}

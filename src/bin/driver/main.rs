#![feature(format_args_capture)]

mod dictionary;
mod driver;
mod naive_rendering_benchmarks;
mod tessellation_benchmarks;
use chrono::Local;
use driver::Driver;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::PathBuf;
use vgpu_bench::util::create_file;

pub fn main() {
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
        .add(tessellation_benchmarks::bench_tessellation_primitives)
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
        .build()
        .run();
}

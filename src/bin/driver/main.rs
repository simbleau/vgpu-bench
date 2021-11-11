#![feature(format_args_capture)]

mod dictionary;
mod driver;
mod naive_rendering_benchmarks;
mod run_options;
mod tessellation_benchmarks;

use std::path::PathBuf;

use chrono::Local;
use const_format::concatcp;
use dictionary::*;
use driver::Driver;
use log::LevelFilter;
use run_options::RunOptions;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use vgpu_bench::util::create_file;

pub fn main() {
    let localtime = Local::now().format("%d%m%Y_%H-%M-%S").to_string();
    let output_dir = PathBuf::from("output/").join(localtime);
    let log_file_path = output_dir.join("trace.log");

    Driver::from(
        RunOptions::builder()
            .output_dir(output_dir.as_path())
            .logger(WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                create_file(log_file_path).unwrap(),
            ))
            .logger(TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ))
            // TODO add bench_tessellation_examples()
            .add(tessellation_benchmarks::bench_tessellation_primitives)
            .add(|o| tessellation_benchmarks::profile_svg_files(o, "assets/svg/examples/"))
            .add(tessellation_benchmarks::profile_svg_primitives)
            .add(|o| naive_rendering_benchmarks::frametimes_svg_files(o, "assets/svg/examples/"))
            .add(naive_rendering_benchmarks::frametimes_svg_primitives)
            .build(),
    )
    .run();
}

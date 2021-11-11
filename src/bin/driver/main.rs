#![feature(format_args_capture)]

mod dictionary;
mod driver;
mod naive_rendering_benchmarks;
mod run_options;
mod tessellation_benchmarks;

use const_format::concatcp;
use dictionary::*;
use driver::Driver;
use log::LevelFilter;
use run_options::RunOptions;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};

const SVG_PRIMITIVES_PATH: &'static str = concatcp!(ASSETS_DIR, SVG, PRIMITIVES);
const SVG_EXAMPLES_PATH: &'static str = concatcp!(ASSETS_DIR, SVG, EXAMPLES);

pub fn main() {
    Driver::from(
        RunOptions::builder()
            .logger(WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                std::fs::File::create("test.log").unwrap(),
            ))
            .logger(TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ))
            // TODO add bench_tessellation_examples()
            .add(|o| tessellation_benchmarks::bench_tessellation_primitives(o))
            //.add(|x| tessellation_benchmarks::profile_svg_examples(SVG_EXAMPLES_PATH))
            //.add(|x| tessellation_benchmarks::profile_svg_primitives())
            //.add(|x| naive_rendering_benchmarks::frametimes_svg_examples(SVG_EXAMPLES_PATH))
            //.add(|x| naive_rendering_benchmarks::frametimes_svg_primitives())
            .build(),
    )
    .run();
}

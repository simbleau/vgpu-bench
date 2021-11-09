#![feature(format_args_capture)]

mod dictionary;
mod naive_rendering_benchmarks;
mod tessellation_benchmarks;

use log::LevelFilter;
use vgpu_bench::driver::{Driver, RunOptions};

pub fn main() {
    // TODO remove SVGTarget
    // TODO change svg_generator::primitives() to svg_generator::primitives::all()/default()
    // TODO add fast methods for primitives
    Driver::from(
        RunOptions::builder()
            .logging(LevelFilter::Trace)
            .add(|| tessellation_benchmarks::bench_tessellation_primitives())
            .add(|| tessellation_benchmarks::profile_svg_examples())
            .add(|| tessellation_benchmarks::profile_svg_primitives())
            .add(|| naive_rendering_benchmarks::frametimes_svg_examples())
            .add(|| naive_rendering_benchmarks::frametimes_svg_primitives())
            .build(),
    )
    .run();
}

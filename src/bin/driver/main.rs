#![feature(format_args_capture)]

mod dictionary;
mod naive_rendering_benchmarks;
mod tessellation_benchmarks;

use log::LevelFilter;
use vgpu_bench::driver::{Driver, RunOptions};

pub fn main() {
    Driver::from(
        RunOptions::builder()
            .logging(LevelFilter::Trace)
            .add(|| tessellation_benchmarks::profile_svg_examples())
            .add(|| tessellation_benchmarks::profile_svg_primitives())
            .add(|| naive_rendering_benchmarks::frametimes_svg_examples())
            .build(),
    )
    .run();

    /*
    let mut primitives = svg_generator::primitives();

    TODO convert to builder
    // Time primitive tessellation
    let path = concatcp![PRIMITIVES_OUTPUT_DIR, "tessellation.csv"];
    perform("primitive tessellation timing", path, || {
        tessellation::benching::tessellating::write_primitive_tessellation_times(
            &primitives,
            path,
            10000,
            1000,
            5,
        )
        .unwrap();
    });

    TODO convert to builder
    // Time naive rendering primitives
    let path = concatcp![PRIMITIVES_OUTPUT_DIR, "naive_frametimes.csv"];
    perform("primitive flat render timing", path, || {
        let mut renderer = NaiveRenderer::new();
        rendering::benching::timing::write_flat_frametimes_primitives(
            &mut renderer,
            &primitives,
            1,
            path,
            100,
        )
        .unwrap();
    });
    */
}

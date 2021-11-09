#![feature(format_args_capture)]

mod dictionary;
mod tessellation_benchmarks;

use ::rendering_util;
use ::svg_generator;
use ::tessellation_util;
use log::LevelFilter;

use const_format::concatcp;
use naive_renderer::NaiveRenderer;
use std::{fs::File, io::Write, path::PathBuf};
use vgpu_bench::{
    benchmarks::tessellation::profile::SVGProfiler,
    driver::{Driver, RunOptions},
};

pub fn main() {
    Driver::from(
        RunOptions::builder()
            .logging(LevelFilter::Trace)
            .add(|| tessellation_benchmarks::profile_svg_examples())
            .add(|| tessellation_benchmarks::profile_svg_primitives())
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
    // Time naive rendering SVG examples
    let path = concatcp![EXAMPLES_OUTPUT_DIR, "naive_frametimes.csv"];
    perform("SVG example flat render timing", path, || {
        let mut renderer = NaiveRenderer::new();
        rendering::benching::timing::write_flat_frametimes_svgs(
            &mut renderer,
            EXAMPLES_ASSETS_DIR,
            path,
            100,
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

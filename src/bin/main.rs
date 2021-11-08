use ::rendering;
use ::svg_generator;
use ::tessellation;

use const_format::concatcp;
use naive_renderer::NaiveRenderer;
use std::{fs::File, io::Write, path::PathBuf};
use vgpu_bench::{
    benchmarks::tessellation::profile::SVGProfiler,
    driver::{Driver, RunOptions},
};

const OUTPUT_DIR: &'static str = "output/data/";
const SVG_OUTPUT_DIR: &'static str = concatcp![OUTPUT_DIR, "svg/"];
const PRIMITIVES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "primitives/"];
const EXAMPLES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "examples/"];

const ASSETS_DIR: &'static str = "assets/";
const SVG_ASSETS_DIR: &'static str = concatcp![ASSETS_DIR, "svg/"];
const PRIMITIVES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "primitives/"];
const EXAMPLES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "examples/"];

pub fn main() {
    Driver::from(
        RunOptions::builder()
            .add(|| {
                let input_dir_path = EXAMPLES_ASSETS_DIR;
                let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
                perform("SVG example profiling", output_path, || {
                    vgpu_bench::benchmarks::tessellation::profile_dir_with_output(
                        input_dir_path,
                        output_path,
                    )
                    .unwrap();
                });
            })
            .add(|| {
                let input_dir_path = PRIMITIVES_ASSETS_DIR;
                let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];
                perform("SVG primitive profiling", output_path, || {
                    vgpu_bench::benchmarks::tessellation::profile_dir_with_output(
                        input_dir_path,
                        output_path,
                    )
                    .unwrap();
                });
            })
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

fn perform(action_message: &'static str, path: &'static str, action: impl Fn() -> ()) {
    print!("Performing {}...", action_message);
    std::io::stdout().flush().expect("Couldn't flush stdout");
    action();
    println!("Complete. Output to {}", path);
}

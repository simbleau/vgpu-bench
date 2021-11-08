use ::rendering;
use ::svg_generator;
use ::tessellation;

use const_format::concatcp;
use mylib::{
    benchmarks::tessellation::profile::SVGProfiler,
    driver::{Driver, RunOptions},
};
use naive_renderer::NaiveRenderer;
use std::{fs::File, io::Write, path::PathBuf};

const OUTPUT_DIR: &'static str = "output/data/";
const SVG_OUTPUT_DIR: &'static str = concatcp![OUTPUT_DIR, "svg/"];
const PRIMITIVES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "primitives/"];
const EXAMPLES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "examples/"];

const ASSETS_DIR: &'static str = "assets/";
const SVG_ASSETS_DIR: &'static str = concatcp![ASSETS_DIR, "svg/"];
const PRIMITIVES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "primitives/"];
const EXAMPLES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "examples/"];

pub fn main() {
    // Declare primitives being used...
    let mut primitives = svg_generator::primitives();

    // TODO : Use an options builder pattern... For now this is decent enough.
    let run_opts = RunOptions::builder()
        .add(|| {
            // Profile SVG Examples
            print!("Performing {}...", "SVG example tessellation profiling");
            std::io::stdout().flush().expect("Couldn't flush stdout");
            // Options
            let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
            let profiler = SVGProfiler::new()
                .writer(csv::Writer::from_path(output_path).unwrap())
                .assets(EXAMPLES_ASSETS_DIR.into(), false)
                .backend(tessellation::backends::default());
            // Run
            mylib::benchmarks::tessellation::profile::profile(profiler).unwrap();
            println!("Complete. Output to {}", output_path);
        })
        .add(|| {
            // Profile SVG Primitive Examples
            print!("Performing {}...", "SVG primitive tessellation profiling");
            std::io::stdout().flush().expect("Couldn't flush stdout");
            // Options
            let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];
            let profiler = SVGProfiler::new()
                .writer(csv::Writer::from_path(output_path).unwrap())
                .assets(PRIMITIVES_ASSETS_DIR.into(), false)
                .backend(tessellation::backends::default());
            // Run
            mylib::benchmarks::tessellation::profile::profile(profiler).unwrap();
            println!("Complete. Output to {}", output_path);
        });

    /*

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

    let d = Driver::from(run_opts.build());
    d.run();
}

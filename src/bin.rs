use ::rendering;
use ::tessellation;
use const_format::concatcp;
use mylib::{
    benchmarks::tessellation::profile::SVGProfiler,
    driver::{Driver, RunOptions},
};
use naive_renderer::NaiveRenderer;
use std::{fs::File, io::Write, path::PathBuf};
use svg_gen::Primitive;

const OUTPUT_DIR: &'static str = "output/data/";
const SVG_OUTPUT_DIR: &'static str = concatcp![OUTPUT_DIR, "svg/"];
const PRIMITIVES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "primitives/"];
const EXAMPLES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "examples/"];

const ASSETS_DIR: &'static str = "assets/";
const SVG_ASSETS_DIR: &'static str = concatcp![ASSETS_DIR, "svg/"];
const PRIMITIVES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "primitives/"];
const EXAMPLES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "examples/"];

pub fn main() {
    // TODO : Use an options builder pattern... For now this is decent enough.
    let run_opts = RunOptions::builder();

    // Declare primitives being used...
    let mut primitives: Vec<(String, Primitive)> = Vec::new();
    primitives.push((String::from("triangle"), Primitive::Triangle));
    primitives.push((
        String::from("quadratic_bezier_curve"),
        Primitive::BezierCurve,
    ));
    primitives.push((
        String::from("cubic_bezier_curve"),
        Primitive::CubicBezierCurve,
    ));

    // Profile SVG examples
    let output: PathBuf = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"].into();
    let output_file = File::create(output).unwrap();
    let csv_writer = csv::Writer::from_writer(output_file);
    let backend = tessellation::backends::default();
    let profiler = SVGProfiler::new()
        .writer(csv_writer)
        .assets(EXAMPLES_ASSETS_DIR.into(), false)
        .backend(backend);
    mylib::benchmarks::tessellation::profile::profile(profiler).unwrap();
    let run_opts = run_opts.add(|| {
        // Don't do anything
        //mylib::benchmarks::tessellation::profile::profile(profiler);
    });

    /*
    TODO convert to builder
    let path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
    perform("SVG example profiling", path, || {
        tessellation::benching::profiling::write_svg_profiles(EXAMPLES_ASSETS_DIR, path).unwrap();
    });

    TODO convert to builder
    // Profile primitive examples
    let path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];
    perform("primitive profiling", path, || {
        tessellation::benching::profiling::write_svg_profiles(PRIMITIVES_ASSETS_DIR, path).unwrap();
    });

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

fn perform(action_message: &'static str, path: &'static str, action: impl Fn() -> ()) {
    print!("Performing {}...", action_message);
    std::io::stdout().flush().expect("Couldn't flush stdout");
    action();
    println!("Complete. Output to {}", path);
}

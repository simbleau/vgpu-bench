#![allow(dead_code)]
use std::io::Write;

use const_format::concatcp;

extern crate tess;

const OUTPUT_DIR: &'static str = "output/data/";
const SVG_OUTPUT_DIR: &'static str = concatcp![OUTPUT_DIR, "svg/"];
const PRIMITIVES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "primitives/"];
const EXAMPLES_OUTPUT_DIR: &'static str = concatcp![SVG_OUTPUT_DIR, "examples/"];

const ASSETS_DIR: &'static str = "assets/";
const SVG_ASSETS_DIR: &'static str = concatcp![ASSETS_DIR, "svg/"];
const PRIMITIVES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "primitives/"];
const EXAMPLES_ASSETS_DIR: &'static str = concatcp![SVG_ASSETS_DIR, "examples/"];

pub fn analyze() {
    let debug = true;
    if !debug {
        todo!();
    }

    // Goal A: Tessellation Analysis
    //
    // Tessellation time vs. primitives
    //bench_primitive_tessellation();
    // Profile of artifacts
    //profile_svg_examples();
    // Render time of SVGs
    render_svg_examples();

    // TODO: Render time of flattened primitives
    // TODO: Render time of hundreds of flattened real world examples
    // TODO: Tessellation tolerance vs. error

    // Goal B: Optimization Analysis
    //
    // TODO: Render time vs. primitives
    // TODO: Render time vs. primitives at scale
    // with metrics (CPU util, RAM, GPU util, VRAM)
    // TODO: Render time of hundreds of real world examples
    // TODO: Render time of real world examples
    // TODO: Render time stability

    // Goal C: Compliance
    //
    // TODO: Compliance of SVG
    // TODO: Compliance of Path Data

    println!("Analysis Complete.")
}

fn perform(action_message: &'static str, action: impl Fn() -> ()) {
    print!("Performing {}...", action_message);
    std::io::stdout().flush().expect("Couldn't flush stdout");
    action();
    println!("Complete.");
}

fn perform_with_output(action_message: &'static str, path: &'static str, action: impl Fn() -> ()) {
    print!("Performing {}...", action_message);
    std::io::stdout().flush().expect("Couldn't flush stdout");
    action();
    println!("Complete. Output to {}", path);
}

fn profile_svg_examples() {
    let path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
    perform_with_output("SVG profiling", path, || {
        tess::benching::profile_svgs(EXAMPLES_ASSETS_DIR, path).unwrap();
    });
}

fn render_svg_examples() {
    let path = concatcp![EXAMPLES_OUTPUT_DIR, "renders.csv"];
    perform_with_output("SVG rendering", path, || {
        tess::benching::render_svgs(EXAMPLES_ASSETS_DIR, path).unwrap();
    });
}

fn bench_primitive_tessellation() {
    println!("Benching primitive tessellation...");
    // Triangles
    let path = concatcp![PRIMITIVES_OUTPUT_DIR, "time_triangles.csv"];
    perform_with_output("triangle tessellation benchmarking", path, || {
        tess::benching::time_primitive(
            "triangle".to_owned(),
            svg_gen::Primitive::Triangle,
            path,
            5,
        )
        .unwrap();
    });

    // Quadratic Curves
    let path = concatcp![PRIMITIVES_OUTPUT_DIR, "time_curves.csv"];
    perform_with_output("quadratic curve tessellation benchmarking", path, || {
        tess::benching::time_primitive(
            "quadratic bezier curve".to_owned(),
            svg_gen::Primitive::BezierCurve,
            path,
            5,
        )
        .unwrap();
    });

    // Cubic Curves
    let path = concatcp![PRIMITIVES_OUTPUT_DIR, "time_cubic_curves.csv"];
    perform_with_output("cubic curve tessellation benchmarking", path, || {
        tess::benching::time_primitive(
            "cubic bezier curve".to_owned(),
            svg_gen::Primitive::CubicBezierCurve,
            path,
            5,
        )
        .unwrap();
    });
}

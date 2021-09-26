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
    // Profile of artifacts
    profile_svg_examples();
    // Tessellation time vs. primitives
    bench_primitive_tessellation();
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

fn profile_svg_examples() {
    print!("Profiling svg examples...");
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
    tess::benching::profile_svgs(EXAMPLES_ASSETS_DIR, output_path).unwrap();
    println!("Complete. Output to {}", output_path);
}

fn bench_primitive_tessellation() {
    println!("Benching primitive tessellation...");
    print!("Benching triangles...");
    let output = concatcp![PRIMITIVES_OUTPUT_DIR, "time_triangles.csv"];
    tess::benching::time_primitive(
        "triangle".to_owned(),
        svg_gen::Primitive::Triangle,
        output,
        10,
    )
    .unwrap();
    println!("Complete. Output to {}.", output);
    print!("Benching curves...");
    let output = concatcp![PRIMITIVES_OUTPUT_DIR, "time_curves.csv"];
    tess::benching::time_primitive(
        "quadratic bezier curve".to_owned(),
        svg_gen::Primitive::BezierCurve,
        output,
        10,
    )
    .unwrap();
    println!("Complete. Output to {}.", output);
}

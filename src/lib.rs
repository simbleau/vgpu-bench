use const_format::concatcp;

extern crate tess;
const OUTPUT_FOLDER_NAME: &'static str = "output/";
const ASSETS_FOLDER_NAME: &'static str = "assets/";
const SVG_PRIMITIVES_FOLDER_NAME: &'static str = concatcp![ASSETS_FOLDER_NAME, "svg-primitives/"];

pub fn analyze() {
    let debug = true;
    if !debug {
        todo!();
    }

    // Goal A: Tessellation Analysis
    //
    // Tessellation time vs. primitives
    let svg_dir = SVG_PRIMITIVES_FOLDER_NAME;
    let output_file = concatcp![OUTPUT_FOLDER_NAME, "tess_primitives.csv"];
    println!("Input:{}\nOutput:{}", svg_dir, output_file);
    tess::benching::write_time_tessellation(svg_dir, output_file).unwrap();
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
}

use const_format::concatcp;

extern crate tess;
const OUTPUT_FOLDER_NAME: &'static str = "output/data/";
const ASSETS_FOLDER_NAME: &'static str = "assets/";
const SVG_PRIMITIVES_FOLDER_NAME: &'static str = concatcp![ASSETS_FOLDER_NAME, "svg-primitives/"];

pub fn analyze() {
    let debug = true;
    if !debug {
        todo!();
    }

    // Goal A: Tessellation Analysis
    //
    // Profile of artifacts
    let svg_dir = SVG_PRIMITIVES_FOLDER_NAME;
    let output_file = concatcp![OUTPUT_FOLDER_NAME, "tess_data.csv"];
    print!("Profiling primitive tessellation...");
    tess::benching::profile_svgs(svg_dir, output_file).unwrap();
    println!("Complete.");
    println!("\tOutput to {}", output_file);
    // Tessellation time vs. primitives
    let svg_dir = SVG_PRIMITIVES_FOLDER_NAME;
    let output_file = concatcp![OUTPUT_FOLDER_NAME, "tess_primitives.csv"];
    print!("Benching primitive tessellation...");
    tess::benching::write_time_tessellation(svg_dir, output_file).unwrap();
    println!("Complete.");
    println!("\tOutput to {}", output_file);
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

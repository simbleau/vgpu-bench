use std::io::Write;

use crate::dictionary::{EXAMPLES_ASSETS_DIR, EXAMPLES_OUTPUT_DIR};
use crate::dictionary::{PRIMITIVES_ASSETS_DIR, PRIMITIVES_OUTPUT_DIR};
use const_format::concatcp;

pub fn profile_svg_examples() {
    print!("Performing SVG example profiling...");
    std::io::stdout().flush().expect("Couldn't flush stdout");

    let input_dir_path = EXAMPLES_ASSETS_DIR;
    let output_path = concatcp![EXAMPLES_OUTPUT_DIR, "profiles.csv"];
    vgpu_bench::benchmarks::tessellation::profile_dir_with_output(input_dir_path, output_path)
        .unwrap();

    println!("Complete. Output to {}", output_path);
}

pub fn profile_svg_primitives() {
    print!("Performing SVG primitive profiling...");
    std::io::stdout().flush().expect("Couldn't flush stdout");

    let input_dir_path = PRIMITIVES_ASSETS_DIR;
    let output_path = concatcp![PRIMITIVES_OUTPUT_DIR, "profiles.csv"];
    vgpu_bench::benchmarks::tessellation::profile_dir_with_output(input_dir_path, output_path)
        .unwrap();

    println!("Complete. Output to {}", output_path);
}

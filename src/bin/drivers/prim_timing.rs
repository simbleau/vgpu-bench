use clap::{App, Arg};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::Path;
use svg_generator::Primitive;
use vgpu_bench::{
    benchmarks::tessellation::TimeSVGPrimitiveTessellation,
    driver::Driver,
    util::{self, create_or_append},
};

pub fn main() {
    // Get arguments
    let matches = App::new("Primitive Timer")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Select an output directory (ex: ./output/)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Select a folder of assets as input (ex: ./input/)")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Sanitize args
    let output_dir = Path::new(matches.value_of("output").unwrap());
    std::fs::create_dir_all(output_dir).expect(
        format!("could not create dir: '{}'", output_dir.display()).as_str(),
    );
    let input_dir = Path::new(matches.value_of("input").unwrap());
    assert!(
        input_dir.exists() && input_dir.is_dir(),
        "input path does not exist"
    );

    let bm = TimeSVGPrimitiveTessellation::new()
        .to_csv("prim_tess_times")
        .to_plot("prim_tess_times")
        .trials(100)
        .primitive_count(1_000)
        .primitive_count(2_000)
        .primitive_count(3_000)
        .primitive_count(4_000)
        .primitive_count(5_000)
        .primitive_count(6_000)
        .primitive_count(7_000)
        .primitive_count(8_000)
        .primitive_count(9_000)
        .primitive_count(10_000)
        .primitive(Primitive::Triangle)
        .primitive(Primitive::BezierCurve)
        .primitive(Primitive::CubicBezierCurve)
        .backend(tessellation_util::backends::default());

    // Run driver
    Driver::builder()
        .on_error_panic(true)
        .output_dir(output_dir)
        .logger(TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ))
        .logger(WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            create_or_append(output_dir.join("trace.log")).unwrap(),
        ))
        .add(bm)
        .build()
        .run();
}

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
        .get_matches();

    // Sanitize args
    let output_dir = Path::new(matches.value_of("output").unwrap());
    std::fs::create_dir_all(output_dir).expect(
        format!("could not create dir: '{}'", output_dir.display()).as_str(),
    );

    let mut bm = TimeSVGPrimitiveTessellation::new()
        .to_csv("prim_tess_times")
        .to_plot("prim_tess_times")
        .trials(5)
        .primitive(Primitive::Triangle)
        .primitive(Primitive::BezierCurve)
        .primitive(Primitive::CubicBezierCurve)
        .backend(tessellation_util::backends::default());

    let mut cnt = 1;
    let cnt_max = 2_u32.pow(19);
    while cnt <= cnt_max {
        bm = bm.primitive_count(cnt);
        cnt = cnt * 2;
    }

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

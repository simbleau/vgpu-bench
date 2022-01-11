use clap::{App, Arg};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::Path;
use vgpu_bench::{
    benchmarks::tessellation::ProfileSVGFiles,
    driver::Driver,
    util::{self, create_or_append},
};

pub fn main() {
    // Get arguments
    let matches = App::new("Tessellation Memory Test Benchmark Driver")
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
        .add(
            ProfileSVGFiles::new()
                .to_csv("stress_profiles")
                .backend(tessellation_util::backends::default())
                .assets(util::get_files(input_dir, false)),
        )
        .build()
        .run();
}

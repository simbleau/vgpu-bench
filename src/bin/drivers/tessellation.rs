use anyhow::Result;
use clap::{Arg, Command};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::Path;
use vgpu_bench::{
    benchmarks::tessellation::{
        ProfileSVGFiles, ProfileSVGPrimitives, TimeSVGPrimitiveTessellation,
    },
    util::{self, io::create_or_append},
    Driver,
};

pub fn main() -> Result<()> {
    // Get arguments
    let matches = Command::new("Tessellation Benchmark Driver")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .arg(
            Arg::new("output")
                .short('o')
                .help("Select an output directory (ex: ./output/)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
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
                .to_csv("file_profiles")
                .to_plot("file_profiles")
                .backend(tessellation_util::backends::default())
                .assets(util::io::get_files(input_dir, false))
                .try_into()?,
        )
        .add(
            ProfileSVGPrimitives::new()
                .to_csv("primitive_profiles")
                // TODO Plotting support
                .backend(tessellation_util::backends::default())
                .primitives(svg_generator::primitives::default())
                .primitive_count(10)
                .primitives_counts((100..=500).step_by(100 as usize))
                .try_into()?,
        )
        .add(
            TimeSVGPrimitiveTessellation::new()
                .to_csv("primitive_tessellation")
                .to_plot("primitive_tessellation")
                .backend(tessellation_util::backends::default())
                .primitives(svg_generator::primitives::default())
                .primitives_counts((100..=1000).step_by(100 as usize))
                .trials(10)
                .try_into()?,
        )
        // TODO TimeSVGFileTessellation
        .build()
        .run()
        .unwrap();

    Ok(())
}

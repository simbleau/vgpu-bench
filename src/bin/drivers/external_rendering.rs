use anyhow::Result;
use clap::{App, Arg};
use log::LevelFilter;
use renderer::ffi::ExternalRenderer;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::Path;
use vgpu_bench::{
    benchmarks::rendering::TimeSVGFileRendering,
    util::{self, create_or_append},
    Driver,
};

pub fn main() -> Result<()> {
    // Get arguments
    let matches = App::new("External Library Rendering Benchmark Driver")
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
            Arg::new("renderer")
                .short('r')
                .help(
                    "Select a compliant rendering library (ex: ./renderer.so)",
                )
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
    let renderer_path = Path::new(matches.value_of("renderer").unwrap());
    assert!(
        renderer_path.exists() && renderer_path.is_file(),
        "renderer path does not exist"
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
            TimeSVGFileRendering::new()
                .to_csv("file_frametimes")
                .to_plot("file_frametimes")
                .renderer(Box::new(unsafe {
                    ExternalRenderer::from(renderer_path).unwrap()
                }))
                .assets(util::get_files(input_dir, false))
                .frames(100)
                .try_into()?,
        )
        .build()
        .run();

    Ok(())
}

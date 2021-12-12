#![feature(format_args_capture)]

use clap::{App, Arg};
use log::LevelFilter;
use renderer::c::CRenderer;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::Path;
use vgpu_bench::{
    benchmarks::{
        rendering::{
            TimeNaiveSVGFileRendering, TimeNaiveSVGPrimitiveRendering,
            TimeSVGFileRendering,
        },
        tessellation::{
            ProfileSVGFiles, ProfileSVGPrimitives, TimeSVGPrimitiveTessellation,
        },
    },
    driver::Driver,
    util::{self, create_file},
};

pub fn main() {
    let matches = App::new("Benchmark Driver")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("Runs arbitrary benchmarks sequentially.")
        // TODO verbose argument
        // TODO on error panic argument
        // TODO logger arguments
        .arg(
            Arg::with_name("output")
                .index(1)
                .help("Select an output directory (ex: ./output/)")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let output_dir = Path::new(matches.value_of("output").unwrap());
    if output_dir.exists() {
        eprintln!("Output path does not exist: '{}'", output_dir.display());
        std::process::exit(1);
    }

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
            create_file(output_dir.join("trace.log")).unwrap(),
        ))
        .add(
            TimeNaiveSVGFileRendering::new()
                .to_csv("naive_file_frametimes")
                .to_plot("naive_file_frametimes")
                .frames(500)
                .backend(tessellation_util::backends::default())
                .assets(util::get_files("assets/svg/examples", false)),
        )
        .add(
            TimeNaiveSVGPrimitiveRendering::new()
                .to_csv("naive_primitive_frametimes")
                .to_plot("naive_primitive_frametimes")
                .backend(tessellation_util::backends::default())
                .frames(500)
                .primitives(svg_generator::primitives::default())
                .primitive_count(1),
        )
        .add(
            ProfileSVGFiles::new()
                .to_csv("file_profiles")
                .to_plot("file_profiles")
                .backend(tessellation_util::backends::default())
                .assets(util::get_files("assets/svg/examples", false)),
        )
        .add(
            ProfileSVGPrimitives::new()
                .to_csv("primitive_profiles")
                // TODO Plotting support
                .backend(tessellation_util::backends::default())
                .primitives(svg_generator::primitives::default())
                .primitive_count(10)
                .primitives_counts((100..=500).step_by(100 as usize)),
        )
        .add(
            TimeSVGPrimitiveTessellation::new()
                .to_csv("primitive_tessellation")
                .to_plot("primitive_tessellation")
                .backend(tessellation_util::backends::default())
                .primitives(svg_generator::primitives::default())
                .primitives_counts((100..=1000).step_by(100 as usize))
                .trials(10),
        )
        /*
        .add(
            TimeSVGFileRendering::new()
                .to_csv("file_frametimes")
                .to_plot("file_frametimes")
                .renderer(Box::new(
                    CRenderer::from("ffi/examples/cpp/renderer.so".into())
                        .unwrap(),
                ))
                .assets(util::get_files("assets/svg/examples", false))
                .frames(100),
        )
        */
        // TODO TimeSVGFileTessellation
        .build()
        .run();
}

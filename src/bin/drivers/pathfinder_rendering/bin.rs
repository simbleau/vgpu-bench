mod render_glue;
use anyhow::Result;
use clap::{App, Arg};
use log::LevelFilter;
use render_glue::PathfinderImpl;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode, WriteLogger};
use std::path::Path;
use vgpu_bench::{
    benchmarks::rendering::TimeSVGFileRendering,
    util::{self, io::create_or_append},
    Driver,
};

pub fn main() -> Result<()> {
    // Get arguments
    let matches = App::new("Pathfinder Rendering Benchmark Driver")
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
    // Due to very complex reasons currently with Pathfinder, we can only handle
    // 1 SVG per renderer instance as it is arduous to re-use the renderer.
    for asset in util::io::get_files(input_dir, false) {
        {
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
                        .to_csv("file_frametimes_pathfinder")
                        .renderer(Box::new(PathfinderImpl::new(asset.clone())))
                        .asset(asset.clone())
                        .frames(100)
                        .try_into()?,
                )
                .build()
                .run();
        }
    }
    // We must call the plotter manually after due to aforementioned complex
    // reasons.
    {
        let benchmark_dir = output_dir.join("benchmarks");
        let csv_path = benchmark_dir.join("file_frametimes_pathfinder.csv");
        let _proc_output = util::exec::call_program(
            "python3",
            [
                "tools/plotter/plot_frametimes_files.py",
                csv_path.to_str().unwrap(),
                benchmark_dir.to_str().unwrap(),
                "file_frametimes_pathfinder",
            ],
        )
        .expect("Plotting failed");
        log::info!(
            "output plot to '{}'",
            benchmark_dir.join("file_frametimes_pathfinder").display()
        );
    }

    Ok(())
}

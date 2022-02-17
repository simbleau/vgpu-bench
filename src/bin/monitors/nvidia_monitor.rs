#![feature(iter_intersperse)]

use clap::{App, Arg};
use std::{io::Write, path::Path};
use vgpu_bench::util;

pub fn main() {
    let matches = App::new("NVIDIA Nsight Systems Monitor")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("Runs an input program under NVIDIA Nsight Systems and consolidates output.")
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
                .help("Select an input program (ex: ./program.sh)")
                .takes_value(true)
                .required(true),
        )
        .arg(
             Arg::new("app_args")
                .multiple_occurrences(true)
        )
        .get_matches();

    // Sanitize args
    let output_dir = Path::new(matches.value_of("output").unwrap());
    std::fs::create_dir_all(output_dir).expect(
        format!("could not create dir: '{}'", output_dir.display()).as_str(),
    );
    let input_path = Path::new(matches.value_of("input").unwrap());
    assert!(
        input_path.exists() && input_path.is_file(),
        "input path does not exist"
    );

    let output_name = "nvidia";

    print!("Running nsys driver...");
    std::io::stdout().flush().unwrap();

    let report_name = output_dir.join(output_name).display().to_string();
    let application_name = input_path.display().to_string();
    let mut args = vec![
        "profile",
        // ===== FLAGS =====
        // Export a SQLite Database as well
        "--export",
        "sqlite",
        // Sample GPU metrics
        "--gpu-metrics-device=all",
        "--gpu-metrics-frequency",
        "20000",
        // Choose output report file name
        "-o",
        &report_name,
        // Overwrite reports if they exist
        "--force-overwrite",
        "true",
        // ===== APPLICATION =====
        &application_name,
    ];
    let app_args = matches.values_of("app_args").unwrap();
    if app_args.len() > 0 {
        args.extend(app_args);
    }

    util::exec::call_program("nsys", args).unwrap();
    println!("Done");

    print!("Converting output...");
    std::io::stdout().flush().unwrap();
    let args = [
        "export",
        // ===== FLAGS =====
        // Choose output file name
        "-o",
        &output_dir
            .join(output_name.to_owned() + ".json")
            .display()
            .to_string(),
        // Choose output file format
        "--type",
        "json",
        // Separated JSON strings for easier parsing
        "--separate-strings",
        "true",
        // ===== QDREP FILE =====
        &output_dir
            .join(output_name.to_owned() + ".nsys-rep")
            .display()
            .to_string(),
    ];
    util::exec::call_program("nsys", args).unwrap();
    println!("Done");

    println!("Execution finished. Exiting...");
}

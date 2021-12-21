#![feature(iter_intersperse)]

use clap::{App, Arg};
use std::{io::Write, path::Path};

pub fn main() {
    let matches = App::new("NVIDIA Nsight Systems Driver")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("Runs an input program under NVIDIA Nsight Systems and consolidates output.")
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
                .help("Select an input program (ex: ./program.sh)")
                .takes_value(true)
                .required(true),
        )
        .arg(
             Arg::with_name("app_args")
                .multiple(true)
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
        // Sample CPU
        "-s",
        "cpu",
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

    vgpu_bench::util::call_program("nsys", args).unwrap();
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
            .join(output_name.to_owned() + ".qdrep")
            .display()
            .to_string(),
    ];
    vgpu_bench::util::call_program("nsys", args).unwrap();
    println!("Done");

    println!("Execution finished. Exiting...");
}

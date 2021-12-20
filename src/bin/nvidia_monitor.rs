use clap::{App, Arg};
use std::{io::Write, path::Path};
use vgpu_bench::nvidia_monitor::NvidiaDriver;

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
                .required(false),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Select an input program (ex: ./program.sh)")
                .takes_value(true)
                .required(true),
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

    print!("Running nsys driver...");
    std::io::stdout().flush().unwrap();
    let driver = NvidiaDriver::new(input_path, output_dir);
    driver.run();
    println!("Done");

    println!("Converting output...");
    std::io::stdout().flush().unwrap();
    driver.convert("json");
    println!("Done");

    println!("Execution finished. Exiting...");
}

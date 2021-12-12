use std::path::{Path, PathBuf};

use chrono::Local;
use clap::{App, Arg};

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
                .help("Select an input program")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // Get input program
    let input_path = Path::new(matches.value_of("input").unwrap());
    if input_path.exists() {
        eprintln!("Input path does not exist: '{}'", input_path.display());
        std::process::exit(1);
    }

    // Get output directory
    let parent_dir = PathBuf::from("output/");
    let output_dir = match matches.is_present("output") {
        true => parent_dir.join(matches.value_of("output").unwrap()),
        false => {
            let timestamp = Local::now().format("%d%m%Y_%H-%M-%S").to_string();
            parent_dir.join(timestamp)
        }
    };
}

extern crate clap;
mod commands;
mod primitives;
mod writer;

use crate::primitives::Primitive;
use crate::writer::Writer;
use clap::{App, Arg};
use std::path::PathBuf;

#[derive(Debug)]
struct Options {
    svg: bool,
    primitive: Primitive,
    output: PathBuf,
    count: i32,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            svg: false,
            primitive: Primitive::Circle,
            output: PathBuf::new(),
            count: 1,
        }
    }
}

fn main() {
    let matches = App::new("Test data generator")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("Generates test data files based on given parameters.")
        .arg(
            Arg::with_name("SVG")
                .short("s")
                .long("svg")
                .help("Returns output in SVG format")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("RAW")
                .short("r")
                .long("raw")
                .help("Returns output as raw path data")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count")
                .help("Select the amount of primitives to output")
                .short("c")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("primitive")
                .short("p")
                .help("Determines the primitive to use")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Select a file name to output")
                .takes_value(true)
                .required(false)
                .index(2),
        )
        .get_matches();

    let mut options = Options::default();

    // Check if SVG is requested
    if matches.is_present("SVG") {
        options.svg = true;
    }

    // Get the requested primitive for output
    let requested_prim = matches.value_of("primitive").unwrap().to_lowercase();
    options.primitive = match requested_prim.as_str() {
        "l" | "line" => Primitive::Line,
        "t" | "triangle" => Primitive::Triangle,
        "p" | "polygon" => Primitive::Polygon,
        "c" | "circle" => Primitive::Circle,
        "e" | "ellipsoid" => Primitive::Ellipsoid,
        "b" | "bezigon" => Primitive::QuadraticBezigon,
        "cb" | "cbezigon" => Primitive::CubicBezigon,
        _ => panic!("Unknown primitive: '{}'", requested_prim),
    };

    // Check if count is requested
    if matches.is_present("count") {
        let count = matches.value_of("count").unwrap();
        options.count = match count.parse::<i32>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };
    }

    // Get output path
    if matches.is_present("output") {
        options.output = PathBuf::from(matches.value_of("output").unwrap());
    }

    // Print options
    // TODO - Remove (Used for debug)
    println!("{:?}", options);

    // Write data
    let mut writer = Writer::default();
    writer.write_primitives(&options.primitive, options.count);

    // Output data
    if options.output.to_str().unwrap().is_empty() {
        println!("{}", writer.get_document());
    } else {
        writer.write_document(options.output.as_path()).unwrap();
    }
}

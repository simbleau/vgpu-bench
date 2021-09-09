extern crate clap;
extern crate dynfmt;

mod commands;
mod primitives;
mod writer;

use crate::primitives::Primitive;
use crate::writer::Writer;
use clap::{App, Arg};
use std::path::PathBuf;

#[derive(Debug)]
struct Options {
    primitive: Primitive,
    output: PathBuf,
    count: i32,
    rotate: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            primitive: Primitive::Line,
            output: PathBuf::new(),
            count: 1,
            rotate: false,
        }
    }
}

fn main() {
    let matches = App::new("SVG file generator")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("Generates SVG files for given shapes and parameters.")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Print verbose information")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("rotate")
                .short("r")
                .long("rotate")
                .help("Rotates all primitives outputted")
                .takes_value(false)
                .required(false),
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

    // Check if rotation is requested
    if matches.is_present("rotate") {
        options.rotate = true;
    }

    // Get the requested primitive for output
    let requested_prim = matches.value_of("primitive").unwrap().to_lowercase();
    options.primitive = match requested_prim.as_str() {
        "l" | "line" => Primitive::Line,
        "t" | "triangle" => Primitive::Triangle,
        "p" | "polygon" => Primitive::Polygon,
        "c" | "curve" => Primitive::BezierCurve,
        "cc" | "cubiccurve" => Primitive::CubicBezierCurve,
        "b" | "bezigon" => Primitive::Bezigon,
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

    // Print debug options
    if matches.is_present("verbose") {
        println!("{:?}", options);
    }

    // Write data
    let mut writer = Writer::default();
    writer.write_primitives(options.primitive, options.count, options.rotate);

    // Output data
    if options.output.to_str().unwrap().is_empty() {
        println!("{}", writer.get_document());
    } else {
        writer.write_document(options.output.as_path()).unwrap();
    }
}

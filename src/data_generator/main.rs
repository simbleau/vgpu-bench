extern crate clap;
mod writer;
use clap::{App, Arg};
use writer::Writer;

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
            Arg::with_name("primitive")
                .possible_values(&["all", "triangle"])
                .help("Determines the primitive to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    if matches.is_present("SVG") {
        println!("SVG");
    } else {
        println!("RAW");
    }

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!(
        "Using primitive: {}",
        matches.value_of("primitive").unwrap()
    );

    let writer = Writer::default();
    println!("Test XML:\n{}", writer.get_document());
}

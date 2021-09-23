extern crate clap;
extern crate dynfmt;

mod commands;
mod primitives;
mod writer;

use crate::primitives::Primitive;
use crate::writer::Writer;
use clap::{App, Arg};
use std::path::PathBuf;

pub fn generate_svg(primitive: Primitive, count: i32, rotate: bool) -> String {
    // Get data
    let mut writer = Writer::default();
    writer.write_primitives(primitive, count, rotate);

    // Return document    
    writer.get_document()
}

pub fn output_svg(primitive: Primitive, count: i32, rotate: bool, path: PathBuf) -> Result<(), std::io::Error> {
        // Get data
        let mut writer = Writer::default();
        writer.write_primitives(primitive, count, rotate);
    
        // Return document    
        Ok(writer.write_document(&path)?)
}
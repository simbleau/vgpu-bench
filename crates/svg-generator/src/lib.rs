extern crate clap;
extern crate dynfmt;

mod primitives;
pub use primitives::Primitive;
mod writer;
use std::{fs::File, io::Write, path::Path};
use writer::Writer;

pub fn primitives() -> Vec<Primitive> {
    let mut primitives: Vec<Primitive> = Vec::new();
    primitives.push(Primitive::Triangle);
    primitives.push(Primitive::BezierCurve);
    primitives.push(Primitive::CubicBezierCurve);
    primitives
}

pub fn generate_svg(primitive: Primitive, count: u32, rotate: bool) -> String {
    let mut writer = Writer::default();
    writer.write_primitives(primitive, count, rotate);
    writer.get_document()
}

pub fn output_svg(
    primitive: Primitive,
    count: u32,
    rotate: bool,
    path: &Path,
) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    let contents = generate_svg(primitive, count, rotate);
    file.write_all(contents.as_bytes())?;
    Ok(())
}

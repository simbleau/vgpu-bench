extern crate xmlwriter;
use std::{fs::File, io::Write, path::Path};
use xmlwriter::*;

use crate::{commands, primitives::Primitive};

const VIEW_MIN: i32 = 0;
const VIEW_MAX: i32 = 100;

pub struct Writer {
    /// The XML Writer object
    xml_writer: XmlWriter,
}

impl Default for Writer {
    fn default() -> Self {
        Writer::new()
    }
}

impl Writer {
    pub fn new() -> Self {
        let opt = Options {
            use_single_quote: true,
            ..Options::default()
        };

        let mut w = XmlWriter::new(opt);
        w.start_element("svg");
        w.write_attribute("xmlns", "http://www.w3.org/2000/svg");
        w.write_attribute_fmt(
            "viewBox",
            format_args!("{} {} {} {}", VIEW_MIN, VIEW_MIN, VIEW_MAX, VIEW_MAX),
        );

        Writer { xml_writer: w }
    }

    pub fn write_primitives(&mut self, p: &Primitive, count: i32) {
        match p {
            Primitive::Line => self.write_lines(count),
            Primitive::Triangle => todo!(),
            Primitive::Polygon => todo!(),
            Primitive::Circle => todo!(),
            Primitive::Ellipsoid => todo!(),
            Primitive::QuadraticBezigon => todo!(),
            Primitive::CubicBezigon => todo!(),
        };
    }

    fn write_lines(&mut self, _count: i32) {
        self.xml_writer.start_element("path");

        // Get data for path
        let mut data = String::new();

        // Start line
        data.push_str(commands::MOVE_TO);

        // Translate line
        // TODO - Avoid occlusion by spacing the lines out using `count`
        data.push_str("50 10 ");
        data.push_str(commands::VERTICAL_LINE_TO);
        data.push_str("50 90 ");

        // End line
        data.push_str(commands::CLOSE_PATH);

        self.xml_writer.write_attribute("d", &data);
        self.xml_writer.write_attribute("stroke", "black");
        self.xml_writer.write_attribute("stroke-width", "1");
    }

    pub fn get_document(self) -> String {
        self.xml_writer.end_document()
    }

    #[allow(dead_code)]
    pub fn write_document(self, path: &Path) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        let contents = self.xml_writer.end_document();
        file.write_all(contents.as_bytes())?;
        Ok(())
    }
}

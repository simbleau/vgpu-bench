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
            Primitive::Triangle => self.write_triangles(count),
            Primitive::Polygon => todo!(),
            Primitive::Circle => todo!(),
            Primitive::Ellipsoid => todo!(),
            Primitive::QuadraticBezigon => todo!(),
            Primitive::CubicBezigon => todo!(),
        };
    }

    fn write_lines(&mut self, count: i32) {
        let size = (count as f32).sqrt() as i32;
        let square_size: f32 = (VIEW_MAX - VIEW_MIN) as f32 / size as f32;
        let padding: f32 = 0.1;
        let offset: f32 = square_size * padding;
        let center: f32 = square_size / 2.0;
        let line_length: f32 = square_size - (square_size * padding * 2.0);
        let line_thickness: f32 = (VIEW_MAX - VIEW_MIN) as f32 / size as f32 / 5.0f32;

        for row in 0..size {
            let x = row as f32 * square_size;
            for col in 0..size {
                let y = col as f32 * square_size;

                self.xml_writer.start_element("path");
                // Get data for path
                let mut data = String::new();
                // Start line
                data.push_str(commands::MOVE_TO);
                // Translate line
                if (row + col) % 2 == 0 {
                    // Vertical line
                    data.push_str(format!("{} {} ", x + center, y + offset).as_str());
                    data.push_str(commands::LINE_TO);
                    data.push_str(format!("{} {} ", x + center, y + line_length + offset).as_str());
                } else {
                    data.push_str(format!("{} {} ", x + offset, y + center).as_str());
                    data.push_str(commands::LINE_TO);
                    data.push_str(format!("{} {} ", x + line_length + offset, y + center).as_str());
                }
                // End line
                data.push_str(commands::CLOSE_PATH);

                self.xml_writer.write_attribute("d", &data);
                self.xml_writer.write_attribute("stroke", "black");
                self.xml_writer
                    .write_attribute("stroke-width", &line_thickness);
                self.xml_writer.end_element();
            }
        }
    }

    fn write_triangles(&mut self, count: i32) {
        let size = (count as f32).sqrt() as i32;
        let square_size: f32 = (VIEW_MAX - VIEW_MIN) as f32 / size as f32;
        let padding: f32 = 0.1;
        let offset: f32 = square_size * padding;
        let center: f32 = square_size / 2.0;
        let line_length: f32 = square_size - (square_size * padding * 2.0);

        for row in 0..size {
            let x = row as f32 * square_size;
            for col in 0..size {
                let y = col as f32 * square_size;

                self.xml_writer.start_element("path");
                // Get data for path
                let mut data = String::new();
                // Start line
                data.push_str(commands::MOVE_TO);
                // Translate line
                if (row + col) % 2 == 0 {
                    // Triange type 1
                    data.push_str(format!("{} {} ", x + center, y + offset).as_str());
                    data.push_str(commands::LINE_TO);
                    data.push_str(format!("{} {} ", x + offset, y + line_length + offset).as_str());
                    data.push_str(commands::LINE_TO);
                    data.push_str(
                        format!("{} {} ", x + offset + line_length, y + line_length + offset)
                            .as_str(),
                    );
                } else {
                    // Triange type 2
                    data.push_str(format!("{} {} ", x + center, y + line_length + offset).as_str());
                    data.push_str(commands::LINE_TO);
                    data.push_str(format!("{} {} ", x + offset, y + offset).as_str());
                    data.push_str(commands::LINE_TO);
                    data.push_str(format!("{} {} ", x + offset + line_length, y + offset).as_str());
                }
                // End line
                data.push_str(commands::CLOSE_PATH);

                self.xml_writer.write_attribute("d", &data);
                self.xml_writer.end_element();
            }
        }
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

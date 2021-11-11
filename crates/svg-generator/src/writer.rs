extern crate xmlwriter;
use dynfmt::{Format, SimpleCurlyFormat};
use xmlwriter::*;

use crate::primitives::Primitive;

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

    pub fn write_primitives(
        &mut self,
        primitive: Primitive,
        count: u32,
        rotate: bool,
    ) {
        let size: i32 = (count as f32).sqrt() as i32;
        let square_size: f32 = (VIEW_MAX - VIEW_MIN) as f32 / size as f32;
        let padding: f32 = 0.1;
        let offset: f32 = square_size / 2f32;

        // Collect the primitive vertices
        let mut verts = Vec::<f32>::new();
        for row in 0..size {
            let x = row as f32 * square_size;
            for col in 0..size {
                let y = col as f32 * square_size;

                for (vert_x, vert_y) in &primitive.unit() {
                    let mut vx = vert_x.to_owned();
                    let mut vy = vert_y.to_owned();
                    // Rotate the primitive vertices
                    if rotate {
                        // Pre-processing rotation values here
                        let index = row * size + col + 1;
                        let theta =
                            index as f32 / (size * size) as f32 * 3.14f32;
                        let cos_theta = (theta as f32).cos();
                        let sin_theta = (theta as f32).sin();
                        // Rotate points
                        vx = vert_x * cos_theta - vert_y * sin_theta;
                        vy = vert_y * cos_theta + vert_x * sin_theta;
                    }

                    // Scale vertices
                    vx *= square_size * (1f32 - padding * 4f32);
                    vy *= square_size * (1f32 - padding * 4f32);

                    // Offset the vertices to their position in the grid
                    vx += x + offset;
                    vy += y + offset;

                    verts.push(vx);
                    verts.push(vy);
                }
            }
        }

        // Write the data
        for row in 0..size {
            for col in 0..size {
                self.xml_writer.start_element("path");

                // Build the path data

                // The index where vertices for this primitive begin
                let group_start: usize = (row * size + col) as usize;

                // Get the vertices as formatting arguments
                // 2D points have 2 parts
                let artifacts = primitive.vertices() * 2;
                let mut arg_buffer: Vec<f32> = vec![0f32; artifacts];
                for i in 0..artifacts {
                    *arg_buffer.get_mut(i).unwrap() =
                        verts[group_start * artifacts + i];
                }
                let args = arg_buffer.as_slice();

                // Format the path data
                let template = &primitive.path_data_template();
                let data = SimpleCurlyFormat.format(template, args).unwrap();

                // Attach the path data
                self.xml_writer.write_attribute("d", &data);

                // Specific rules for primitives
                if let Primitive::Line = primitive {
                    self.xml_writer.write_attribute("stroke", "black");
                    let line_thickness: f32 =
                        (VIEW_MAX - VIEW_MIN) as f32 / size as f32 / 5.0f32;
                    self.xml_writer
                        .write_attribute("stroke-width", &line_thickness);
                }

                self.xml_writer.end_element();
            }
        }
    }

    pub fn get_document(self) -> String {
        self.xml_writer.end_document()
    }
}

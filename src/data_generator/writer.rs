extern crate xmlwriter;
use xmlwriter::*;

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
        w.write_attribute_fmt("viewBox", format_args!("{} {} {} {}", 0, 0, 100, 100));

        // TODO - Remove this. Only for testing.
        w.start_element("text");
        // We can write any object that implements `fmt::Display`.
        w.write_attribute("x", &10);
        w.write_attribute("y", &20);
        w.write_text_fmt(format_args!("length is {}", 5));

        Writer { xml_writer: w }
    }

    pub fn get_document(self) -> String {
        let x = self.xml_writer;
        x.end_document()
    }
}

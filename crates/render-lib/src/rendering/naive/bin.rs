use clap::{App, Arg};
use render_lib::rendering::naive::TriangleRenderer;
use std::path::PathBuf;
use tess_lib::targets::{SVGDocument, SVGFile};

fn main() {
    let app = App::new("SVG Tessellation Renderer")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("A basic renderer for SVGs using triangulation.")
        .arg(
            Arg::with_name("file path")
                .help("Select an SVG file to render")
                .takes_value(true)
                .required(true)
                .index(1), // Args start at 1
        )
        .get_matches();

    // Get file
    let file_path: &PathBuf = &app.value_of("file path").unwrap().into();
    let file = SVGFile::from(file_path);
    let svg_document = &SVGDocument::from(&file);

    // Run demo
    let mut tessellator = tess_lib::backends::default();
    let mut renderer = TriangleRenderer::new();
    renderer
        .init_with_svg(tessellator.as_mut(), svg_document)
        .unwrap();
    renderer.run().unwrap();
}

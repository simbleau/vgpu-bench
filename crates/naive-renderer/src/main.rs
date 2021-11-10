use clap::{App, Arg};
use naive_renderer::TriangleRenderer;
use renderer::targets::{SVGDocument, SVGFile};
use std::path::PathBuf;

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
    let svg = SVGDocument::from(file);

    // Run demo
    let mut tessellator = tess_lib::backends::default();
    let mut renderer = TriangleRenderer::new();
    renderer.init_with_svg(tessellator.as_mut(), &svg).unwrap();
    renderer.run().unwrap();
}

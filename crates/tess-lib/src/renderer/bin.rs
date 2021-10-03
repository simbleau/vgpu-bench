use tess_lib::{
    renderer::Renderer,
    targets::{SVGDocument, SVGFile},
};

fn main() {
    // Get indices and verts
    let file = SVGFile {
        path: "/home/spencer/School/Thesis/vgpu-bench/assets/svg/examples/NASA.svg".into(),
    };
    let svg_document = &SVGDocument::from(&file);
    let mut tessellator = tess_lib::backends::default();

    let mut r = Renderer::new();
    r.init_with_svg(tessellator.as_mut(), svg_document).unwrap();
    r.toggle_wireframe();
    r.run();
}

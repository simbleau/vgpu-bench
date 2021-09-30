use tess_lib::{
    backends::LyonTessellator,
    renderer::{util::get_globals, Renderer},
    targets::{SVGDocument, SVGFile},
    Tessellator,
};

fn main() {
    // Get indices and verts
    let file = SVGFile {
        path: "/home/spencer/School/Thesis/vgpu-bench/assets/svg/examples/NASA.svg".into(),
    };
    let svg_doc = SVGDocument::from(&file);

    let scene = get_globals(&svg_doc);
    let mut tess = LyonTessellator::new();
    tess.init(&svg_doc);
    let data = *tess.get_tessellate_data().unwrap();

    let mut r = Renderer::new();
    r.init(scene, data).unwrap();
    let results = r.run(120).unwrap();
    println!("{:?}", results.frame_times);

    println!("Finished");
}

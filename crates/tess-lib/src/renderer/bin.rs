use std::{thread, time::Duration};

use tess_lib::{
    backends::LyonTessellator,
    renderer::{get_globals, Renderer},
    targets::{SVGDocument, SVGFile},
    Tessellator,
};

fn main() {
    let r = Renderer::new();

    // Get indices and verts
    let file = SVGFile {
        path: "/home/spencer/School/Thesis/vgpu-bench/assets/svg/examples/Ghostscript_Tiger.svg"
            .into(),
    };
    let svg_doc = SVGDocument::from(&file);

    let scene = get_globals(&svg_doc);
    let mut tess = LyonTessellator::new();
    tess.init(&svg_doc);
    let data = *tess.get_tessellate_data().unwrap();
    let handle = thread::spawn(move || {
        r.run(scene, data, 1);
    });
    handle.join().unwrap();

    println!("Finished");
}

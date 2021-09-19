use std::{cell::RefCell, rc::Rc};

use lyon_tessellator::LyonTessellator;
use tess_lib::TessellationTarget;

mod lyon_tessellator;
mod tess_lib;

pub fn analyze() {
    let debug = true;
    if !debug {
        todo!();
    }

    // Goal A: Tessellation Analysis
    //
    // TODO: Tessellation time vs. primitives

    let tessellator = LyonTessellator::new();
    let tess_rc = Rc::new(RefCell::new(tessellator));
    let mut target = TessellationTarget {
        tessellator: tess_rc.clone(),
        path: "/home/spencer/School/Thesis/vgpu-bench/assets/ASU.svg".to_string(),
    };
    let (t1, t2) = target.time_tessellation();
    println!(
        "Pre-processing: {}ms\nTessellation: {}ms",
        t1.as_millis(),
        t2.as_millis()
    );

    // TODO: Render time of flattened primitives
    // TODO: Render time of hundreds of flattened real world examples
    // TODO: Tessellation tolerance vs. error

    // Goal B: Optimization Analysis
    //
    // TODO: Render time vs. primitives
    // TODO: Render time vs. primitives at scale
    // with metrics (CPU util, RAM, GPU util, VRAM)
    // TODO: Render time of hundreds of real world examples
    // TODO: Render time of real world examples
    // TODO: Render time stability

    // Goal C: Compliance
    //
    // TODO: Compliance of SVG
    // TODO: Compliance of Path Data
}

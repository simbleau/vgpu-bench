use std::{cell::RefCell, fs, io, path::PathBuf, rc::Rc};

use lyon_tessellator::LyonTessellator;
use tess_lib::TessellationTarget;
use walkdir::WalkDir;

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

    let tessellator = Rc::new(RefCell::new(LyonTessellator::new()));
    let mut target = TessellationTarget {
        tessellator: tessellator.clone(),
        path: "/home/spencer/School/Thesis/vgpu-bench/assets/Ghostscript_Tiger.svg".to_string(),
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

fn get_files<P>(path: P, recursive: bool) -> Result<Vec<PathBuf>, io::Error>
where
    P: Into<PathBuf>,
{
    let mut walkdir = WalkDir::new(path.into());
    if !recursive {
        walkdir = walkdir.max_depth(1);
    }
    let files = walkdir
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.path().is_file())
        .map(|p| p.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    Ok(files)
}

#[cfg(test)]
mod tests {
    #[test]
    fn list_files() {
        use crate::get_files;
        for file in get_files("assets/", false).unwrap() {
            println!("{:?}", file);
        }
    }
}

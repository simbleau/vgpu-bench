use std::{io, path::PathBuf};

extern crate tess;

use tess::{LyonTessellator, TessellationTarget, Tessellator};
use walkdir::WalkDir;

pub fn analyze() {
    let debug = true;
    if !debug {
        todo!();
    }

    // Goal A: Tessellation Analysis
    //
    // Tessellation time vs. primitives
    // impl Iterator<Item = Box<dyn &Tessellator>>
    let mut csv_wtr = csv::Writer::from_writer(io::stdout());
    csv_wtr
        .write_record(&["tessellator", "file", "prep time", "tessellation time"])
        .unwrap();

    let mut lyon = LyonTessellator::new();
    let tessellators: Vec<Box<&mut dyn Tessellator>> = vec![Box::new(&mut lyon)];

    let files = get_files("assets/", false).unwrap();

    for tessellator in tessellators {
        let col1_name = tessellator.name();
        let tesser: &mut dyn Tessellator = *tessellator;
        for file in &files {
            let mut target = TessellationTarget {
                path: file.to_path_buf(),
            };
            let (t1, t2) = target.time_tessellation(Box::new(tesser));
            let col2_filename = file.file_name().unwrap().to_str().unwrap().to_owned();
            let col3_prep_time = t1.as_millis().to_string();
            let col4_tess_time = t2.as_millis().to_string();
            csv_wtr
                .write_record(&[
                    col1_name.to_owned(),
                    col2_filename,
                    col3_prep_time,
                    col4_tess_time,
                ])
                .unwrap();
        }
    }
    csv_wtr.flush().unwrap();

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

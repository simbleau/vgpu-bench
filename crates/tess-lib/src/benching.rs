use crate::{LyonTessellator, TessellationResult, TessellationTarget, Tessellator};

use std::{fs::File, io, path::PathBuf};

use walkdir::WalkDir;

pub fn write_time_tessellation<P>(svg_dir: P, output: P) -> Result<(), io::Error>
where
    P: Into<PathBuf>,
{
    let file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(file);
    let mut lyon = LyonTessellator::new();
    let tessellators: Vec<Box<&mut dyn Tessellator>> = vec![Box::new(&mut lyon)];

    let files = get_files(svg_dir, false).unwrap();

    for tessellator in tessellators {
        let tesselator: &mut dyn Tessellator = *tessellator; // Unwrap & Shadow
        for file in &files {
            let mut target = TessellationTarget {
                path: file.to_path_buf(),
            };
            let (init_time, tess_time, vertices, indices) =
                target.time_tessellation(Box::new(tesselator));

            let result = TessellationResult {
                tessellator: tesselator.name().to_owned(),
                filename: file.file_name().unwrap().to_str().unwrap().to_owned(),
                vertices,
                indices,
                init_time: init_time.as_millis() as i32,
                tess_time: tess_time.as_millis() as i32,
            };
            csv_wtr.serialize(result)?;
        }
    }
    csv_wtr.flush()?;

    Ok(())
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
        for file in super::get_files("../../assets/", false).unwrap() {
            println!("{:?}", file);
        }
    }
}

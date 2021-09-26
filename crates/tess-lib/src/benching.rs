use crate::artifacts::{SVGProfileResult, TimeResult};
use crate::targets::{SVGDocument, SVGFile, TessellationTarget};
use crate::{LyonTessellator, Tessellator};

use std::{fs::File, io, path::PathBuf};

use walkdir::WalkDir;

fn backends() -> Vec<Box<dyn Tessellator>> {
    let mut tessellators: Vec<Box<dyn Tessellator>> = vec![];
    tessellators.push(Box::new(LyonTessellator::new()));
    tessellators
}

pub fn time_primitives<P>(output: P, trials: i32) -> Result<(), io::Error>
where
    P: Into<PathBuf>,
{
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, tessellate the files
    for mut backend in backends() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow
                                                           // Tessellate each primitive up to an amount
        let primitives = vec![svg_gen::Primitive::Triangle];
        let counts = std::iter::once(1).chain((10..=50).step_by(10));
        for primitive in primitives {
            for count in counts.clone() {
                for _ in 0..trials {
                    let mut target: SVGDocument =
                        SVGDocument::from(svg_gen::generate_svg(primitive, count, true));
                    let (init_time, tess_time) = target.time(Box::new(backend));

                    let result = TimeResult {
                        tessellator: backend.name().to_owned(),
                        filename: format!("triangle-{}", count),
                        init_time: init_time.as_nanos() as i32,
                        tess_time: tess_time.as_nanos() as i32,
                    };
                    csv_wtr.serialize(result)?;
                }
            }
        }
    }

    csv_wtr.flush()?;

    Ok(())
}

pub fn profile_svgs<P>(svg_dir: P, output: P) -> Result<(), io::Error>
where
    P: Into<PathBuf>,
{
    let files = get_files(svg_dir, false).unwrap();
    let output_file = File::create(output.into())?;
    let mut csv_wtr = csv::Writer::from_writer(output_file);

    // For each backend, retrieve the file profiles
    for mut backend in backends() {
        let backend: &mut dyn Tessellator = &mut *backend; // Unwrap & Shadow

        // Retrieve the profile from files and record the results
        for file in &files {
            let target: SVGFile = file.into();
            let (vertices, indices) = target.get_data(Box::new(backend));

            let result = SVGProfileResult {
                tessellator: backend.name().to_owned(),
                filename: file.file_name().unwrap().to_str().unwrap().to_owned(),
                vertices,
                indices,
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

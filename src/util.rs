use csv::Writer;
use walkdir::WalkDir;

use crate::Result;
use std::{ffi::OsStr, fs::File, path::PathBuf};

pub fn csv_writer<P>(path: P) -> Result<Writer<File>>
where
    P: Into<PathBuf>,
{
    let output_file = create_file(path)?;
    Ok(csv::Writer::from_writer(output_file))
}

pub fn create_file<P>(path: P) -> Result<File>
where
    P: Into<PathBuf>,
{
    // Make sure path can exist by making parent directories if they are missing
    let output_path: PathBuf = path.into();
    let parent_dir = output_path.parent().expect("Path must have a parent");
    std::fs::create_dir_all(parent_dir)?;

    Ok(std::fs::File::create(output_path)?)
}

pub fn get_files<P>(dir: P, recursive: bool) -> Vec<PathBuf>
where
    P: Into<PathBuf>,
{
    let mut walkdir = WalkDir::new(dir.into());
    if !recursive {
        walkdir = walkdir.max_depth(1);
    }
    let files = walkdir
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|f| f.path().is_file())
        .map(|p| p.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    files
}

pub fn get_files_with_extension<P, S>(
    dir: P,
    recursive: bool,
    ext: &S,
) -> Vec<PathBuf>
where
    P: Into<PathBuf>,
    S: AsRef<OsStr> + ?Sized,
{
    get_files(dir, recursive)
        .into_iter()
        .filter(|path| path.extension() == Some(OsStr::new(ext)))
        .collect()
}

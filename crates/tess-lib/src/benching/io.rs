use super::error::{BenchingError, Result};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_files<P>(path: P, recursive: bool) -> Result<Vec<PathBuf>>
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

    if files.len() == 0 {
        return Err(BenchingError::Logic("No files found"));
    }

    Ok(files)
}

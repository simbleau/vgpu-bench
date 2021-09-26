use std::{io, path::PathBuf};

use walkdir::WalkDir;

mod primitive_benching;
pub use primitive_benching::time_primitive;

mod profiling;
pub use profiling::profile_svgs;

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

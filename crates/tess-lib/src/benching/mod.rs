use std::path::PathBuf;

use walkdir::WalkDir;

mod profiling;
pub use profiling::profile_svgs;

mod rendering;
pub use rendering::render_svgs;

mod tessellating;
pub use tessellating::time_primitive;

mod error;
use error::Result;

fn get_files<P>(path: P, recursive: bool) -> Result<Vec<PathBuf>>
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
        return Err(error::BenchingError::Logic("No files found"));
    }

    Ok(files)
}

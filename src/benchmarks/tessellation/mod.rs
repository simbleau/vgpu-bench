pub mod error;
pub mod profile;

use error::Result;
use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

pub fn profile_dir_with_output<P>(input_dir_path: P, output_path: P) -> Result<()>
where
    P: Into<std::path::PathBuf>,
{
    // Make sure path can exist
    let output_path: PathBuf = output_path.into();
    let parent_dir = output_path.parent().expect("Path must have a parent");
    std::fs::create_dir_all(parent_dir)?;

    // Options
    let profiler = profile::SVGProfiler::new()
        .writer(csv::Writer::from_path(output_path)?)
        .assets(input_dir_path.into(), false)
        .backend(tessellation::backends::default());
    // Run
    Ok(profile::profile(profiler)?)
}

pub mod error;
pub mod profile;

use error::Result;

pub fn profile_dir_with_output<P>(input_dir_path: P, output_path: P) -> Result<()>
where
    P: Into<std::path::PathBuf>,
{
    // Options
    let profiler = profile::SVGProfiler::new()
        .writer(csv::Writer::from_path(output_path.into())?)
        .assets(input_dir_path.into(), false)
        .backend(tessellation::backends::default());
    // Run
    Ok(profile::profile(profiler)?)
}

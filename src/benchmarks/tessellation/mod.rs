pub mod error;
pub mod profile;

use error::Result;

pub fn profile_dir_with_output<P>(input_dir_path: P, output_path: P) -> Result<()>
where
    P: Into<std::path::PathBuf>,
{
    // Options
    let input_files = crate::util::get_files_with_extension(input_dir_path, false, "svg");
    let output_file = crate::util::create_file(output_path).expect("");
    let writer = csv::Writer::from_writer(output_file);
    // Make profiler with options
    let profiler = profile::SVGProfiler::new()
        .writer(writer)
        .assets(input_files)
        .backend(tessellation::backends::default());
    // Run
    Ok(profile::profile(profiler)?)
}

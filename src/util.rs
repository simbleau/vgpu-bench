use super::Result;
use csv::Writer;
use log::{error, trace, warn};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    path::PathBuf,
    process::Output,
};
use walkdir::WalkDir;

pub fn call_python3_program<I, S>(program_path: S, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let program_path = program_path.as_ref();
    let os_str = OsString::from(program_path);
    let mut new_args: Vec<OsString> = Vec::new();
    new_args.push(os_str);
    let args = args.into_iter().map(|x| OsString::from(x.as_ref()));
    new_args.extend(args);

    trace!(
        "running python3 program '{}'",
        program_path.to_string_lossy()
    );
    // Run program
    let output = std::process::Command::new("python3")
        .args(new_args)
        .output()
        .expect("python3 was unable to run. Is `python3` in your PATH?");

    // Check status code
    let output = match output.status.success() {
        true => output,
        false => {
            error!(
                "python3 call to program '{}' exited with failed status ({}) [Stderr: '{}']",
                program_path.to_string_lossy(),
                output.status.to_string(),
                &String::from_utf8_lossy(&output.stderr)
            );
            return Err(anyhow::anyhow!(
                "failed python3 program call ({})",
                output.status.to_string()
            ));
        }
    };

    trace!(
        "completed python3 program '{}' successfully",
        program_path.to_string_lossy()
    );
    Ok(output)
}

pub fn write_csv<P>(
    path: P,
    rows: &Vec<Box<dyn erased_serde::Serialize>>,
) -> Result<()>
where
    P: Into<PathBuf>,
{
    let mut output_path = path.into();
    output_path.set_extension("csv");
    let mut writer = csv_writer(&output_path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn csv_writer_relative<P>(relative_path: P) -> Result<Writer<File>>
where
    P: Into<PathBuf>,
{
    let path = relative_path.into();
    if path.is_relative() {
        Ok(csv_writer(path)?)
    } else {
        Err(anyhow::anyhow!(
            "Argument '{}' is not a relative path",
            path.display()
        ))
    }
}

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

pub fn files_with_extension<I, S>(files: I, ext: &S) -> Vec<PathBuf>
where
    I: IntoIterator,
    I::Item: Into<PathBuf>,
    S: AsRef<OsStr> + ?Sized,
{
    files
        .into_iter()
        .map(Into::into)
        .fold(Vec::new(), |mut vec, pb| {
            if pb.exists()
                && pb.is_file()
                && pb.extension() == Some(OsStr::new(ext))
            {
                vec.push(pb);
            } else {
                warn!(
                    "'{}' is not a .{:?} file; file dropped",
                    pb.display(),
                    OsStr::new(ext).to_string_lossy()
                );
            }
            vec
        })
}

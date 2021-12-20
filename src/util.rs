use super::Result;
use csv::Writer;
use log::{error, trace, warn};
use renderer::targets::{SVGDocument, SVGFile};
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    path::PathBuf,
    process::Output,
};
use walkdir::WalkDir;

pub fn path_to_svg<P>(path: P) -> SVGDocument
where
    P: Into<PathBuf>,
{
    let file = SVGFile::from(&path.into());
    SVGDocument::from(file)
}

pub fn call_program<I, S>(program_path: S, args: I) -> Result<Output>
where
    I: IntoIterator<Item = S>,
    I: Clone,
    S: AsRef<OsStr>,
{
    let program_path = OsString::from(program_path.as_ref());

    trace!(
        "executing process '{} {}'",
        program_path.to_string_lossy(),
        args.clone()
            .into_iter()
            .map(|arg| arg.as_ref().to_string_lossy().to_string())
            .intersperse(" ".to_string())
            .collect::<String>()
    );

    // Run program
    let output = std::process::Command::new(&program_path)
        .args(args)
        .output()
        .expect(
            format!(
                "'{}' was unable to execute, is it in your PATH?",
                program_path.to_string_lossy()
            )
            .as_str(),
        );

    // Check status code
    let output = match output.status.success() {
        true => output,
        false => {
            error!(
                "'{}' exited with failure ({}, err: '{}')",
                program_path.to_string_lossy(),
                output.status.to_string(),
                &String::from_utf8_lossy(&output.stderr)
            );
            return Err(anyhow::anyhow!(
                "'{}' exited with failure ({}, err: '{}')",
                program_path.to_string_lossy(),
                output.status.to_string(),
                &String::from_utf8_lossy(&output.stderr)
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
    let output_file = create_or_append(path)?;
    Ok(csv::Writer::from_writer(output_file))
}

pub fn create_or_append<P>(path: P) -> Result<File>
where
    P: Into<PathBuf>,
{
    // Make sure path can exist by making parent directories if they are missing
    let output_path: PathBuf = path.into();
    let parent_dir = output_path.parent().expect("Path must have a parent");
    std::fs::create_dir_all(parent_dir)?;
    Ok(std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_path)?)
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

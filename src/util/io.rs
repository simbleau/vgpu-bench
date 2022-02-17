use crate::Result;
use anyhow::ensure;
use csv::Writer;
use log::warn;
use std::{
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn dir_exists<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.is_dir() && path.exists() {
        true
    } else {
        false
    }
}

pub fn dir_is_empty<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if dir_exists(path) && path.read_dir().unwrap().peekable().peek().is_none()
    {
        true
    } else {
        false
    }
}

pub fn dir_create_all<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    Ok(std::fs::create_dir_all(path)?)
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
    ensure!(
        path.is_relative(),
        "Argument '{}' is not a relative path",
        path.display()
    );
    Ok(csv_writer(path)?)
}

pub fn csv_writer<P>(path: P) -> Result<Writer<File>>
where
    P: Into<PathBuf>,
{
    let output_file = create_or_append(path)?;
    let mut write_header = true;
    if output_file.metadata().unwrap().len() > 0 {
        write_header = false;
    }
    let writer = csv::WriterBuilder::default()
        .has_headers(write_header)
        .from_writer(output_file);
    Ok(writer)
}

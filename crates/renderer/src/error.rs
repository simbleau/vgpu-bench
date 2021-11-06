use crate::error::RendererError::CppLibraryError;
use std::{error::Error, fmt};

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(Debug)]
pub enum RendererError {
    RustLibraryError(Box<dyn std::error::Error>),
    CppLibraryError(CppRendererError),
}

// C++ Error Glue below
impl<T> From<T> for RendererError
where
    T: Into<CppRendererError>,
{
    fn from(item: T) -> Self {
        CppLibraryError(item.into())
    }
}

#[derive(Debug)]
pub enum CppRendererError {
    Compilation(libloading::Error),
    Initialization(libloading::Error),
    Staging(libloading::Error),
    Rendering(libloading::Error),
    Runtime(i32),
}
impl fmt::Display for CppRendererError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
impl Error for CppRendererError {}

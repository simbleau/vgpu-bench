use std::fmt::Display;

use winit::error::OsError;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(Debug)]
pub enum RendererError {
    RendererNotInitialized,
    FatalRenderingError,
    OsError(OsError),
}

impl Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::RendererNotInitialized => {
                write!(f, "{}", "The renderer was not initialized.")
            }
            RendererError::FatalRenderingError => {
                write!(f, "{}", "An unknown fatal error ocurred.")
            }
            RendererError::OsError(oserr) => write!(f, "{}", oserr),
        }
    }
}

impl From<OsError> for RendererError {
    fn from(item: OsError) -> Self {
        RendererError::OsError(item)
    }
}

impl std::error::Error for RendererError {}

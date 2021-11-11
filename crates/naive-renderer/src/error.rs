use std::fmt::Display;

use winit::error::OsError;

pub type Result<T> = std::result::Result<T, NaiveRendererError>;

#[derive(Debug)]
pub enum NaiveRendererError {
    RendererNotInitialized,
    FatalRenderingError,
    FatalTessellationError(Box<dyn std::error::Error>),
    OsError(OsError),
}

impl Display for NaiveRendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaiveRendererError::RendererNotInitialized => {
                write!(f, "{}", "The renderer was not initialized.")
            }
            NaiveRendererError::FatalRenderingError => {
                write!(
                    f,
                    "{}",
                    "An unknown fatal error ocurred during rendering."
                )
            }
            NaiveRendererError::FatalTessellationError(err) => {
                write!(
                    f,
                    "{}\n{}",
                    "An unknown fatal error ocurred during tessellation.", err
                )
            }
            NaiveRendererError::OsError(oserr) => write!(f, "{}", oserr),
        }
    }
}

impl From<OsError> for NaiveRendererError {
    fn from(item: OsError) -> Self {
        NaiveRendererError::OsError(item)
    }
}

impl std::error::Error for NaiveRendererError {}

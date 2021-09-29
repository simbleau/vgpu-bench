use winit::error::OsError;

pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(Debug)]
pub enum RendererError {
    RendererNotInitialized,
    OsError(OsError),
}

impl From<OsError> for RendererError {
    fn from(item: OsError) -> Self {
        RendererError::OsError(item)
    }
}

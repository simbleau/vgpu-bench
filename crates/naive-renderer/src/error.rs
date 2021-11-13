pub type Result<T> = std::result::Result<T, NaiveRendererError>;

#[derive(thiserror::Error, Debug)]
pub enum NaiveRendererError {
    #[error("the renderer was not initialized.")]
    RendererNotInitialized,
    #[error("an unknown fatal error ocurred during rendering")]
    FatalRenderingError,
    #[error("an fatal error ocurred during tessellation")]
    FatalTessellationError(
        #[source]
        #[from]
        Box<dyn std::error::Error + Send + Sync>,
    ),
    #[error("the renderer incurred an OS-specific error")]
    OsError(
        #[source]
        #[from]
        winit::error::OsError,
    ),
}

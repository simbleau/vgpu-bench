pub type Result<T> = std::result::Result<T, RendererError>;

#[derive(thiserror::Error, Debug)]
pub enum RendererError {
    #[error("unexpected fatal error with Rust renderer")]
    RustLibraryError(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("unexpected fatal error with C/C++ renderer")]
    ExternalLibraryError(
        #[source]
        #[from]
        ExternalRendererError,
    ),
}

#[derive(thiserror::Error, Debug)]
pub enum ExternalRendererError {
    #[error("unable to load external library")]
    LibraryRetrieval(libloading::Error),
    #[error("unable to initialize external library")]
    Initialization(libloading::Error),
    #[error("Unable to stage file")]
    Staging(libloading::Error),
    #[error("Unable to render file")]
    Rendering(libloading::Error),
    #[error("external library returned error code: {0} (expected 0)")]
    Runtime(i32),
}

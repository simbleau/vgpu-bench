pub type Result<T> = std::result::Result<T, BenchingError>;

#[derive(thiserror::Error, Debug)]
pub enum BenchingError {
    #[error("{0}")]
    Logic(&'static str),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    CSV(#[from] csv::Error),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

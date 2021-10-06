use crate::renderer;

pub type Result<T> = std::result::Result<T, BenchingError>;

#[derive(Debug)]
pub enum BenchingError {
    Rendering(renderer::error::RendererError),

    IO(std::io::Error),
    Logic(&'static str),
    CSV(csv::Error),
    Other(Box<dyn std::error::Error>),
}

impl From<renderer::error::RendererError> for BenchingError {
    fn from(item: renderer::error::RendererError) -> Self {
        BenchingError::Rendering(item)
    }
}
impl From<std::io::Error> for BenchingError {
    fn from(item: std::io::Error) -> Self {
        BenchingError::IO(item)
    }
}
impl From<csv::Error> for BenchingError {
    fn from(item: csv::Error) -> Self {
        BenchingError::CSV(item)
    }
}

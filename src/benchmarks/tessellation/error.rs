use crate::benchmarks::tessellation::error::TessellationError::Benching;

pub type Result<T> = std::result::Result<T, TessellationError>;

#[derive(Debug)]
pub enum TessellationError {
    Benching(tessellation::benching::error::BenchingError),
}

impl<T> From<T> for TessellationError
where
    T: Into<tessellation::benching::error::BenchingError>,
{
    fn from(item: T) -> Self {
        Benching(item.into())
    }
}

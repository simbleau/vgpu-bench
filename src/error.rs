use crate::benchmarks::tessellation::error::TessellationError;
use crate::error::VgpuBenchError::Tessellation;

pub type Result<T> = std::result::Result<T, VgpuBenchError>;

#[derive(Debug)]
pub enum VgpuBenchError {
    Tessellation(TessellationError),
}

impl<T> From<T> for VgpuBenchError
where
    T: Into<TessellationError>,
{
    fn from(item: T) -> Self {
        Tessellation(item.into())
    }
}

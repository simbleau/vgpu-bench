use crate::benchmarks::rendering::error::RenderingError::Benching;

pub type Result<T> = std::result::Result<T, RenderingError>;

#[derive(Debug)]
pub enum RenderingError {
    Benching(rendering_util::benching::BenchingError),
}

impl<T> From<T> for RenderingError
where
    T: Into<rendering_util::benching::BenchingError>,
{
    fn from(item: T) -> Self {
        Benching(item.into())
    }
}

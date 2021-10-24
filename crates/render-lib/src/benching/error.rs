pub type Result<T> = std::result::Result<T, BenchingError>;

#[derive(Debug)]
pub enum BenchingError {
    IO(std::io::Error),
    Logic(&'static str),
    CSV(csv::Error),
    Other(Box<dyn std::error::Error>),
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
impl From<Box<dyn std::error::Error>> for BenchingError {
    fn from(item: Box<dyn std::error::Error>) -> Self {
        BenchingError::Other(item)
    }
}

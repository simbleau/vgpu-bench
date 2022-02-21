use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("{0}")]
    Polling(String),
    #[error(transparent)]
    PollingOther(#[from] Box<dyn std::error::Error + Send + Sync>),
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Err(String),
    #[error("CsvIOError:")]
    CsvIO(#[from] csv::Error),
    #[error("stdIOError:")]
    IO(#[from] std::io::Error),
    #[error("json")]
    JsonIO(#[from] serde_json::error::Error)
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Err(s)
    }
}
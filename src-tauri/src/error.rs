use calamine::{XlsxError, XlsError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Err(&'static str),
    #[error("stdIOError: ")]
    IO(#[from] std::io::Error),
    #[error("JsonError: ")]
    JsonIO(#[from] serde_json::error::Error),
    #[error("CalaMineIOError: ")]
    CalaIO(#[from] calamine::Error),
    #[error("XlsxError: ")]
    XlsxError(#[from] XlsxError),
    #[error("XlsError: ")]
    XlsError(#[from] XlsError),
}

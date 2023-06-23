use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("unknown error")]
    Unknown,
}
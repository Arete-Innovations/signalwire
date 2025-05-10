use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignalWireError {
    #[error("HTTP request failed with status: {0}")]
    HttpError(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

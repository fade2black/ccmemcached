use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(&'static str),
    #[error("unknown data store error")]
    Unknown,
}


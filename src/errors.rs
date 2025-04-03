use thiserror::Error;

pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Inavlid command")]
    InvalidCommand,
    #[error("Unexpected command `{0}`")]
    UnexpectedCommand(String),
    #[error("State access error")]
    StateAccessError,
}

use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("failed to read data file")]
    ReadFileError,

    #[error("failed to write data file")]
    WriteFileError,

    #[error("failed to persist file to disk")]
    SyncFileError,

    #[error("failed to open file")]
    OpenFileError,
}

pub type Result<T> = result::Result<T, Errors>;

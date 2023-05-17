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

    #[error("database path is empty")]
    PathEmpty,

    #[error("database options of data file size is incorrect")]
    DataFileSizeError,

    #[error("specific data file not found")]
    DataFileNotFound,

    #[error("failed to find key")]
    KeyNotFound,

    #[error("key is empty")]
    KeyIsEmpty,

    #[error("update index error")]
    IndexUpdateError,
}

pub type Result<T> = result::Result<T, Errors>;

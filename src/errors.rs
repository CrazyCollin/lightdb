use thiserror::Error;

pub type Result<E>=std::result::Result<E,KvError>;

#[derive(Error,Debug)]
pub enum KvError{
    #[error("Not found {0}")]
    NotFound(String),
}

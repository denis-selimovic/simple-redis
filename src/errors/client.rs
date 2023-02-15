use std::fmt::Debug;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ClientError {
    #[error("could not connect to server on port {0}")]
    ConnectionError(u16),
    #[error("could not parse input")]
    InputError,
    #[error("command could not be executed")]
    CommandExecutionError,
}

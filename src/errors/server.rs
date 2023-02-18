use std::fmt::Debug;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ServerError {
    #[error("command could not be parsed")]
    CommandExecutionError,
}

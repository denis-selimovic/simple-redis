use std::fmt::Debug;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum CommandError {
    #[error("expected {1} parameters for command {0}, got {2} instead")]
    WrongParamNumber(String, usize, usize),
    #[error("expected at least {1} parameters for command {0}, got {2} instead")]
    WrongVariableParamNumber(String, usize, usize),
    #[error("invalid param type {1} for command {0}")]
    InvalidParamType(String, usize),
}

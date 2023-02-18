use std::fmt::Debug;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("empty byte sequence")]
    Empty,
    #[error("unknown type with start byte {0}")]
    UnknownStartByte(u8),
    #[error(r#"missing carriage return ('\r') when parsing"#)]
    MissingCR,
    #[error(r#"missing line feed ('\n') when parsing"#)]
    MissingLF,
    #[error("{0} cannot be converted to signed 64-bit integer")]
    IntegerOverflow(String),
    #[error("invalid length {0} for bulk string")]
    InvalidStringLength(i64),
    #[error("expected string length to be {0}, got {1} instead")]
    StringLengthMismatch(usize, usize),
    #[error("invalid array length")]
    InvalidArrayLength,
    #[error("missing end byte")]
    MissingEndByte,
}
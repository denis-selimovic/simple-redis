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
}
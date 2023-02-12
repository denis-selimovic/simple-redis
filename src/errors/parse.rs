use std::fmt::Debug;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum ParsingError {
    #[error(r#"missing carriage return ('\r') when parsing"#)]
    MissingCR,
    #[error(r#"missing line feed ('\n') when parsing"#)]
    MissingLF,
}
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;

pub type ComandResult = Result<Type, CommandError>;
pub type Command = fn(&Type, &mut Storage) -> ComandResult;

use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;

pub type CommandResult = Result<Type, CommandError>;
pub type Command = for<'a, 'b> fn(&'a [Type], &'b mut Storage) -> CommandResult;

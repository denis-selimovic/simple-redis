use crate::errors::command::CommandError;
use crate::protocol::types::Type;

pub type CommandResult = Result<Type, CommandError>;

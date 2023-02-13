use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn delete<'a, 'b>(args: &'a [Type], storage: &'b mut Storage) -> CommandResult {
    if args.len() != 1 {
        return Err(CommandError::WrongParamNumber("DELETE".to_string(), 1, args.len()));
    }

    match &args[0] {
        Type::BulkString(s) | Type::SimpleString(s) => {
            Ok(storage.remove(s))
        },
        _ => Err(CommandError::InvalidParamType("DELETE".to_string(), 1)),
    }
}

use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn set<'a, 'b>(args: &'a [Type], storage: &'b mut Storage) -> CommandResult {
    if args.len() != 2 {
        return Err(CommandError::WrongParamNumber("SET".to_string(), 2, args.len()));
    }

    match &args[0] {
        Type::BulkString(s) | Type::SimpleString(s) => {
            storage.write(s.to_string(), args[1].clone());

            Ok(Type::Integer(1))
        },
        _ => Err(CommandError::InvalidParamType("SET".to_string(), 1)),
    }
}

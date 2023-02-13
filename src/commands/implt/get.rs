use crate::commands::types::ComandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn get<'a, 'b>(args: &'a [Type], storage: &'b mut Storage) -> ComandResult {
    if args.len() != 1 {
        return Err(CommandError::WrongParamNumber("GET".to_string(), 1, args.len()));
    }

    match &args[0] {
        Type::BulkString(s) | Type::SimpleString(s) => {
            let value = storage.read(s);
            
            match value {
                None => Ok(Type::Null),
                Some(v) => Ok(v.clone()),
            }
        },
        _ => Err(CommandError::InvalidParamType("GET".to_string(), 1)),
    }
}

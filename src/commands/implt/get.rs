use std::sync::{Arc, Mutex};

use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn get<'a>(args: &'a [Type], storage: Arc<Mutex<Storage>>) -> CommandResult {
    if args.len() != 1 {
        return Err(CommandError::WrongParamNumber("GET".to_string(), 1, args.len()));
    }

    match &args[0] {
        Type::BulkString(s) | Type::SimpleString(s) => {
            let value = storage.lock().unwrap().read(s);
            
            match value {
                None => Ok(Type::Null),
                Some(v) => Ok(v),
            }
        },
        _ => Err(CommandError::InvalidParamType("GET".to_string(), 1)),
    }
}

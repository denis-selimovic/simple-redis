use std::sync::{Arc, Mutex};

use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn flush<'a>(args: &'a [Type], storage: Arc<Mutex<Storage>>) -> CommandResult {
    if args.len() != 0 {
        return Err(CommandError::WrongParamNumber("FLUSH".to_string(), 0, args.len()));
    }

    Ok(storage.lock().unwrap().flush())
}

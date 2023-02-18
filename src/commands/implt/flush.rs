use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn flush<'a, 'b>(args: &'a [Type], storage: &'b mut Storage) -> CommandResult {
    if args.len() != 0 {
        return Err(CommandError::WrongParamNumber("FLUSH".to_string(), 0, args.len()));
    }

    Ok(storage.flush())
}

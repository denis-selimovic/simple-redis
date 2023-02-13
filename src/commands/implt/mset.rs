use crate::commands::implt::set::set;
use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn mset<'a, 'b>(args: &'a [Type], storage: &'b mut Storage) -> CommandResult {
    if args.len() % 2 != 0  || args.len() == 0 {
        return Err(CommandError::WrongVariableParamNumber("MSET".to_string(), 2, args.len()));
    }

    for i in (0..args.len()).step_by(2) {
        set(&args[i..=i + 1], storage)?;
    }

    Ok(Type::Integer((args.len() / 2) as i64))
}

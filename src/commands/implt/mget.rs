use std::slice::from_ref;
use std::sync::{Arc, Mutex};

use crate::commands::implt::get::get;
use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn mget<'a>(args: &'a [Type], storage: Arc<Mutex<Storage>>) -> CommandResult {
    if args.len() < 1 {
        return Err(CommandError::WrongVariableParamNumber("MGET".to_string(), 1, args.len()));
    }

    let mut res = vec![];

    for arg in args {
        let val = get(from_ref(arg), storage.clone())?;
        res.push(val)
    }

    Ok(Type::Array(res))
}

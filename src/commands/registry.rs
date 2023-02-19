use std::sync::{Arc, Mutex};

use crate::commands::implt::delete::delete;
use crate::commands::implt::flush::flush;
use crate::commands::implt::get::get;
use crate::commands::implt::mget::mget;
use crate::commands::implt::mset::mset;
use crate::commands::implt::set::set;
use crate::commands::types::CommandResult;
use crate::errors::command::CommandError;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub fn execute(op: &str, args: &[Type], storage: Arc<Mutex<Storage>>) -> CommandResult {
    match op {
        "DELETE" => delete(args, storage),
        "FLUSH" => flush(args, storage),
        "GET" => get(args, storage),
        "MGET" => mget(args, storage),
        "MSET" => mset(args, storage),
        "SET" => set(args, storage),
        _ => Err(CommandError::InvalidOp(op.to_string())),
    }
}

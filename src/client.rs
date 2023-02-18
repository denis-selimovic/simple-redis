use async_std::prelude::*;
use async_std::io;
use async_std::net;
use std::marker::Unpin;

use crate::errors::client::ClientError;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;


pub async fn send_commands(mut stream: net::TcpStream ) -> Result<(), ClientError> {
    let mut command_line = io::BufReader::new(io::stdin()).lines();

    while let Some(command) = command_line.next().await {
        match command {
            Err(_) => return Err(ClientError::InputError),
            Ok(command) => {
                match command.strip_suffix("\n") {
                    None => continue,
                    Some(buff) => send_command(&mut stream, buff).await?,
                };
            }
        }
    }

    Ok(())
}

async fn send_command<S>(stream: &mut S, cmd: &str) -> Result<(), ClientError>
where
    S: async_std::io::Write + Unpin
{
    let cmd_args = cmd
        .split_whitespace()
        .map(|s| Type::BulkString(s.to_string()))
        .collect();
    let bytes = serialize(&Type::Array(cmd_args));

    if let Err(_) = stream.write_all(&bytes).await {
        return Err(ClientError::CommandExecutionError);
    }

    Ok(())
}

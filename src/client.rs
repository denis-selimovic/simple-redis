use async_std::prelude::*;
use async_std::io;
use async_std::net;
use std::marker::Unpin;

use crate::errors::client::ClientError;
use crate::protocol::deserializer::deserialize;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;


pub async fn send_commands(mut stream: net::TcpStream ) -> Result<(), ClientError> {
    let mut command_line = io::BufReader::new(io::stdin()).lines();

    while let Some(command) = command_line.next().await {
        match command {
            Err(_) => return Err(ClientError::InputError),
            Ok(command) => send_command(&mut stream, &command).await?
        }
    }

    Ok(())
}

pub async fn receive_replies(stream: net::TcpStream) -> Result<(), ClientError> {
    let mut buff = io::BufReader::new(stream);
    loop {
        let reply = receive_reply(&mut buff).await?;
        println!(">>> {}", reply.to_string());
    }
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

async fn receive_reply<S>(stream: &mut S) -> Result<Type, ClientError>
where
    S: async_std::io::BufRead + Unpin
{
    let mut bytes = vec![];

    match stream.read_until(b'!', &mut bytes).await {
        Err(_) => return Err(ClientError::CommandExecutionError),
        Ok(_) => {
            match deserialize(&mut bytes.into_iter()) {
                Err(_) => return  Err(ClientError::CommandExecutionError),
                Ok(t) => return Ok(t),
            }
        }
    }
}

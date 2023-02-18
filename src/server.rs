use async_std::prelude::*;
use async_std::{io, net};

use crate::commands::registry::execute;
use crate::errors::server::ServerError;
use crate::protocol::deserializer::deserialize;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub async fn execute_commands(stream: net::TcpStream) -> Result<(), ServerError>{
    let mut outbound = stream.clone();
    let mut storage = Storage::new();

    let mut buff = io::BufReader::new(stream);
    loop {
        let reply = execute_command(&mut buff, &mut storage).await?;
        let bytes = serialize(&reply);

        match outbound.write(&bytes).await {
            Err(err) => eprintln!("{}", err.to_string()),
            Ok(n) => println!("{} bytes written", n),
        }
    }
}


async fn execute_command<S>(stream: &mut S, storage: &mut Storage) -> Result<Type, ServerError>
where
    S: async_std::io::BufRead + Unpin
{
    let mut bytes = vec![];
    match stream.read_until(b'!', &mut bytes).await {
        Err(_) => return Err(ServerError::CommandExecutionError),
        Ok(_) => {
            match deserialize(&mut bytes.into_iter()) {
                Err(_) => return  Err(ServerError::CommandExecutionError),
                Ok(command) => {
                    match command {
                        Type::Array(arr) => {
                            if arr.len() == 0 {
                                Ok(Type::Error("missing instruction".to_string()))
                            } else {
                                match &arr[0] {
                                    Type::BulkString(s) | Type::SimpleString(s) => {
                                        match execute(s, &arr[1..], storage) {
                                            Err(err) => Ok(Type::Error(err.to_string())),
                                            Ok(t) => Ok(t),
                                        }
                                    },
                                    _ => Ok(Type::Error("instruction must be encoded as string".to_string())),
                                }
                            }
                        },
                        _ => Ok(Type::Error("command must be encoded as array of strings".to_string())),
                    }
                }
            }
        }
    }
}

use async_std::prelude::*;
use async_std::{io, net, task};
use std::sync::{Arc, Mutex};

use crate::commands::registry::execute;
use crate::errors::server::ServerError;
use crate::protocol::deserializer::deserialize;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub async fn serve(listener: net::TcpListener) {
    let mut new_connections = listener.incoming();
    let storage = Arc::new(Mutex::new(Storage::new()));

    while let Some(socket_res) = new_connections.next().await {
        match socket_res {
            Err(err) => println!("{}", err.to_string()),
            Ok(socket) => {
                let storage = storage.clone();
                task::spawn(async move {
                    println!("Starting command execution for {}", socket.local_addr().unwrap().to_string());
                    let _ = execute_commands(socket, storage).await;
                });
            },
        }
    }
}

async fn execute_commands(stream: net::TcpStream, storage: Arc<Mutex<Storage>>) -> Result<(), ServerError>{
    let mut outbound = stream.clone();

    let mut buff = io::BufReader::new(stream);
    loop {
        let storage = storage.clone();
        let reply = execute_command(&mut buff, storage).await?;
        let bytes = serialize(&reply);

        match outbound.write(&bytes).await {
            Err(err) => eprintln!("{}", err.to_string()),
            Ok(n) => println!("{} bytes written", n),
        }
    }
}


async fn execute_command<S>(stream: &mut S, storage: Arc<Mutex<Storage>>) -> Result<Type, ServerError>
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

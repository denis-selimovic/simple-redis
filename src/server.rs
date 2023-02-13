use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::commands::registry::execute;
use crate::protocol::deserializer::deserialize;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;
use crate::storage::Storage;


pub struct Server {
    storage: Box<Storage>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            storage: Box::new(Storage::new()),
        }
    }

    pub fn run(&mut self) {
        let binding = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in binding.incoming() {
            let mut stream = stream.unwrap();
            self.execute(&mut stream);
        }
    }

    fn execute(&mut self, stream: &mut TcpStream) {
        let mut bytes = vec![];
        let result = stream.read_to_end(&mut bytes);

        let response = match result {
            Err(err) => Type::Error(err.to_string()),
            Ok(_) => self.parse_command(bytes),
        };

        let output = serialize(&response);
        stream.write_all(&output).unwrap();
    }

    pub fn parse_command(&mut self, bytes: Vec<u8>) -> Type {
        let mut iter = bytes.into_iter();
        let command_res = deserialize(&mut iter);

        match command_res {
            Err(err) => Type::Error(err.to_string()),
            Ok(command) => {
                match command {
                    Type::Array(arr) => {
                        if arr.len() == 0 {
                            Type::Error("missing instruction".to_string())
                        } else {
                            match &arr[0] {
                                Type::BulkString(s) | Type::SimpleString(s) => {
                                    match execute(s, &arr[1..], &mut self.storage) {
                                        Err(err) => Type::Error(err.to_string()),
                                        Ok(t) => t,
                                    }
                                },
                                _ => Type::Error("instruction must be encoded as string".to_string()),
                            }
                        }
                    },
                    _ => Type::Error("command must be encoded as array of strings".to_string()),
                }
            }
        }
    }
}

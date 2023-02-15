use std::io::{stdin, Write, Read};
use std::net::TcpStream;

use crate::errors::client::ClientError;
use crate::protocol::deserializer::deserialize;
use crate::protocol::serializer::serialize;
use crate::protocol::types::Type;


pub struct Client {
    port: u16,
}


impl Client {
    pub fn new(port: u16) -> Self {
        Client{ port: port }
    }

    pub fn start(&mut self) -> Result<(), ClientError> {
        let address = format!("127.0.0.1:{}", self.port);
        let stream_res = TcpStream::connect(address);

        if let Err(_) = stream_res {
            return Err(ClientError::ConnectionError(self.port));
        }

        let mut stream = stream_res.ok().unwrap();

        loop {
            let command = self.read_command()?;
            let bytes = serialize(&command);

            let result = self.fetch_result(&mut stream, bytes)?;
            let mut iter = result.into_iter();

            match deserialize(&mut iter) {
                Err(_) => return Err(ClientError::CommandExecutionError),
                Ok(t) => println!("{}", t.to_string())
            };
        }
    }

    fn read_command(&self) -> Result<Type, ClientError> {
        let mut buff = String::new();

        match stdin().read_line(&mut buff) {
            Err(_) => Err(ClientError::InputError),
            Ok(_) => {
                let res = match buff.strip_suffix("\n") {
                    Some(buff) => buff.split_whitespace().map(|s| s.to_string()).collect(),
                    None => vec![],
                };

                let cmd = res.iter().map(|s| Type::BulkString(s.to_string())).collect();
                Ok(Type::Array(cmd))
            },
        }
    }

    fn fetch_result(&self, stream: &mut TcpStream, bytes: Vec<u8>) -> Result<Vec<u8>, ClientError>{
        if let Err(_) = stream.write_all(&bytes) {
            return Err(ClientError::CommandExecutionError);
        }

        let mut response = vec![];
        match stream.read(&mut response) {
            Err(_) => Err(ClientError::CommandExecutionError),
            Ok(_) => Ok(response),
        }
    }
}

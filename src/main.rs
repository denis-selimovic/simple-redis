use std::net::TcpListener;

pub mod commands;
pub mod errors;
pub mod protocol;
pub mod storage;


#[cfg(test)]
mod tests;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        print!("Connection established!");
    }
}

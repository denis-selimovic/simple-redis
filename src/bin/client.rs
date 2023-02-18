use async_std::{net, task};
use futures_lite::future::FutureExt;
use simple_redis::client::{receive_replies, send_commands};


fn main() {
    let address = "127.0.0.1:7878";

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await;

        match socket {
            Err(err) => {
                println!("{}", err.to_string());
                return;
            },
            Ok(socket) => {
                let _ = socket.set_nodelay(true);

                let to_server = send_commands(socket.clone());
                let from_server = receive_replies(socket);

                let _ = from_server.race(to_server).await;
            }
        }
    });
}

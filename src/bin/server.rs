use async_std::{net, task};
use futures_lite::StreamExt;

use simple_redis::server::execute_commands;

fn main() {
    let address = "127.0.0.1:7878";

    task::block_on(async {
        let conn_res = net::TcpListener::bind(address).await;

        match conn_res {
            Err(err) => println!("{}", err.to_string()),
            Ok(listener) => {
                let mut new_connections = listener.incoming();

                while let Some(socket_res) = new_connections.next().await {
                    match socket_res {
                        Err(err) => println!("{}", err.to_string()),
                        Ok(socket) => {
                            task::spawn(async move {
                                println!("Starting command execution for {}", socket.local_addr().unwrap().to_string());
                                let _ = execute_commands(socket).await;
                            });
                        },
                    }
                }
            }
        }
    });
}

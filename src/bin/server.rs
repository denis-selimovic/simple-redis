use async_std::{net, task};

use simple_redis::server::serve;

fn main() {
    let address = "127.0.0.1:7878";

    task::block_on(async {
        let conn_res = net::TcpListener::bind(address).await;

        match conn_res {
            Err(err) => println!("{}", err.to_string()),
            Ok(listener) => {
                serve(listener).await;
            }
        }
    });
}

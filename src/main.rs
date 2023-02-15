pub mod client;
pub mod commands;
pub mod errors;
pub mod protocol;
pub mod server;
pub mod storage;


#[cfg(test)]
mod tests;

use crate::server::Server;
use crate::client::Client;


fn main() {
    let mut c = Client::new(7878);
    let _ = c.start();
}

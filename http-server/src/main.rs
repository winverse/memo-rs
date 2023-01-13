mod http;
mod server;

use http::{Method, Request};
use server::Server;

fn main() {
    let address = String::from("127.0.0.1:8080");

    let server = Server::new(address);
    server.run();
}

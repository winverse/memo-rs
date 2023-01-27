#![allow(dead_code)] // ! 를 추가하게 되면 전체 파일에 적용?
#![warn(unused_variables)]

mod http;
mod server;
mod website_handler;

use http::{Method, Request};
use server::Server;
use website_handler::WebSiteHandler;

fn main() {
    let address = String::from("127.0.0.1:8080");

    let server = Server::new(address);
    server.run(WebSiteHandler);
}

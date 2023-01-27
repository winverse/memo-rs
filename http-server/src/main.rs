#![allow(dead_code)] // ! 를 추가하게 되면 전체 파일에 적용?
#![warn(unused_variables)]

mod http;
mod server;
mod website_handler;

use std::env;

use http::{Method, Request};
use server::Server;
use website_handler::WebSiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    // PUBLIC_PATH=$(pwd)/public cargo run : 이런식으로 실행이 가능함
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let address = String::from("127.0.0.1:8080");
    let server = Server::new(address);
    server.run(WebSiteHandler::new(public_path));
}

use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use dotenv::dotenv;
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.js", get(javascript_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new().into())),
        });

    let address: SocketAddr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()
        .expect("Invalid server address");

    let server = Server::bind(&address).serve(router.into_make_service());

    match server.await {
        Ok(_) => println!("listening on  {}", address),
        Err(e) => eprintln!("server error: {}", e),
    }
    let message: String = String::from("hello");

    println!("message: {}", message);
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let html = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(html)
}

#[axum::debug_handler]
async fn javascript_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("src/index.js").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}

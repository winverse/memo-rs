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

#[derive(Default, Clone)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();
    let app_state = AppState::default();

    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.mjs", get(js_get))
        .route("/index.css", get(css_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(app_state.clone());

    // Update CPU usage in the background
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

            // new scope created
            // 코드 실행이 끝나면 lock이 해제가 되어서 lock time이 0ms가 출력이 됨
            {
                let mut cpus = app_state.cpus.lock().unwrap();
                *cpus = v;
            }

            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }
    });

    let address: SocketAddr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()
        .expect("Invalid server address");

    let server = Server::bind(&address).serve(router.into_make_service());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    } else {
        println!("listening on  {}", address);
    }
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let html = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(html)
}

#[axum::debug_handler]
async fn js_get() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("src/index.mjs").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js)
        .unwrap()
}

#[axum::debug_handler]
async fn css_get() -> impl IntoResponse {
    let css = tokio::fs::read_to_string("src/index.css").await.unwrap();
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(css)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let lock_start = std::time::Instant::now();
    let v = state.cpus.lock().unwrap().clone();
    let lock_elapsed = lock_start.elapsed().as_millis();
    println!("Lock time: {}ms", lock_elapsed);
    Json(v)
}

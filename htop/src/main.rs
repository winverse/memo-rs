use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;

type SnapShot = Vec<f32>;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<SnapShot>,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();

    let (tx, _) = broadcast::channel::<SnapShot>(1);

    let app_state = AppState { tx: tx.clone() };

    let router = Router::new()
        .route("/", get(root_get))
        .route("/index.mjs", get(js_get))
        .route("/index.css", get(css_get))
        // .route("/api/cpus", get(cpus_get))
        .route("/realtime/cpus", get(realtime_cpus_get))
        .with_state(app_state.clone());

    // Update CPU usage in the background
    tokio::task::spawn_blocking(move || {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);
            // std::thread::sleep 은 비동기 코드에서 사용할 수 없는 이유는 이 함수가 현재 스레드를 지정된 시간 동안 재우는데,
            // 비동기 함수에서는 모든 비동기 함수가 같은 스레드에서 실행되기 때문입니다.
            // 따라서 std::thread::sleep 은 비동기 함수에 대해 아무것도 모르고 전체 스레드를 재우게 되어 다른 비동기 작업들도 멈추게 됩니다

            // 비동기 코드에서 스레드를 재우고 싶다면 async_std::task::sleep 과 같은 비동기 버전의 함수를 사용해야 합니다.
            // 이 함수는 std::thread::sleep 과 유사하지만 비동기 작업을 방해하지 않습니다.
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

// #[axum::debug_handler]
// async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
//     let lock_start = std::time::Instant::now();
//     let v = state.cpus.lock().unwrap().clone();
//     let lock_elapsed = lock_start.elapsed().as_millis();
//     println!("Lock time: {}ms", lock_elapsed);
//     Json(v)
// }

#[axum::debug_handler]
async fn realtime_cpus_get(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async {
        realtime_cpus_stream(state, ws).await;
    })
}

async fn realtime_cpus_stream(app_state: AppState, mut ws: WebSocket) {
    let mut rx = app_state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        let payload = serde_json::to_string(&msg).unwrap();
        ws.send(Message::Text(payload)).await.unwrap();
    }
}

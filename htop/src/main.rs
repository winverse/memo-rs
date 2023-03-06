use axum::{extract::State, routing::get, Router, Server};
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
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new().into())),
        });

    let address: SocketAddr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:8081".to_string())
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

async fn root_get() -> &'static str {
    "Hello World"
}

async fn cpus_get(State(state): State<AppState>) -> String {
    use std::fmt::Write;

    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();

    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;
        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} {usage}%").unwrap();
    }

    s
}

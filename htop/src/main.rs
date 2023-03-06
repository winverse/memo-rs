use axum::{extract::State, routing::get, Router, Server};
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new().into())),
        });

    let server = Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(router.into_make_service());
    let address = server.local_addr();

    match server.await {
        Ok(_) => println!("listening on {address}"),
        Err(e) => eprintln!("server error: {}", e),
    }
}

async fn root_get(State(state): State<AppState>) -> String {
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

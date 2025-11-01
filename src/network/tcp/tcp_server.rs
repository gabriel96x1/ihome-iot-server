use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use local_ip_address::local_ip;
use tokio::net::TcpListener;
use crate::network::tcp::websocket::ws_service::ws_communication;

pub async fn run_tcp_server() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ws/communication", get(ws_communication));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));

    let ip = local_ip();
    println!("Web Server running at: {}", ip.unwrap());

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();

    if let Err(e) = axum::serve(listener, app).await {
        println!("Server error: {e}");
    }
}

async fn health_check() -> &'static str {
    "I'm alive!"
}
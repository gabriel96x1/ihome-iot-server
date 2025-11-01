use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;
use crate::network::tcp::websocket::ws_service::handle_ws_communication;

pub async fn run_tcp_server() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/ws/communication", get(handle_ws_communication));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    println!("Web Server running at: {}", addr);

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
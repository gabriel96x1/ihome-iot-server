use std::net::SocketAddr;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::WebSocket;
use axum::response::Response;
use crate::orchestration::incoming_communication_orchestrator::IncomingCommunicationOrchestrator;

pub async fn handle_ws_incoming_communication(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Response {
    ws.on_upgrade(move |socket| handle_ws_socket(socket, addr))
}

async fn handle_ws_socket(socket: WebSocket, addr: SocketAddr) {

    let mut communication_orchestrator = IncomingCommunicationOrchestrator::new().await;

    communication_orchestrator.orchestrate(socket, addr).await;

}
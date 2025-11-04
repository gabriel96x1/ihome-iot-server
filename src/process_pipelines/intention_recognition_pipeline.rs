use axum::extract::ws::WebSocket;
use tokio::net::UdpSocket;
use crate::process_pipelines::steps::receive_audio_step::receive_audio_step;
use crate::process_pipelines::steps::stt_step::stt_step;

pub async fn intention_recognition_pipeline(udp_socket: &UdpSocket, ws: WebSocket, client_addr: std::net::SocketAddr) {
    let audio_path: String = receive_audio_step(udp_socket, ws, client_addr).await;
    let recognized_text: String = stt_step(audio_path).await;
    println!("{}", recognized_text);
}
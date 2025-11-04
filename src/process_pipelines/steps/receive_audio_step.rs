use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use axum::extract::ws::{Message, WebSocket};
use sha2::{Digest, Sha256};
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::sound::wav_utils::save_wav;

pub async fn receive_audio_step(udp_socket: Arc<UdpSocket>, ws: WebSocket, client_addr: SocketAddr) -> String {

    let audio_path = generate_audio_path_from(client_addr);
    let audio_path_clone = audio_path.clone();
    let str_path = audio_path_clone.as_str();

    recording_session_controller(udp_socket, ws, client_addr, str_path).await;

    audio_path

}

fn generate_audio_path_from(client_addr: SocketAddr) -> String {
    let client_ip = client_addr.ip().to_string();
    let client_port = client_addr.port().to_string();
    let client_addr = format!("{}:{}", client_ip, client_port);
    let mut hasher = Sha256::new();
    hasher.update(client_addr);
    let result = hasher.finalize();
    let hash = hex::encode(result);
    format!("tmp/audio-{}.wav", hash)
}

async fn audio_receiver(
    udp_socket: Arc<UdpSocket>,
    audio_data: Arc<Mutex<Vec<i16>>>,
    client_addr: SocketAddr,
    recording_flag: Arc<AtomicBool>
) {
    let mut buf = [0u8; 1024];

    while recording_flag.load(Ordering::SeqCst) {
        match udp_socket.recv_from(&mut buf).await {
            Ok((len, udp_addr)) => {
                if udp_addr.ip() == client_addr.ip() {
                    let mut audio = audio_data.lock().await;
                    for chunk in buf[..len].chunks_exact(2) {
                        let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                        audio.push(sample);
                    }
                }
            }
            Err(e) => {
                eprintln!("UDP recv error: {:?}", e);
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    }
}

async fn recording_session_controller(
    udp_socket: Arc<UdpSocket>,
    mut ws: WebSocket,
    client_addr: SocketAddr,
    audio_path: &str
) {

    let audio_data = Arc::new(Mutex::new(Vec::<i16>::new()));
    let recording_flag = Arc::new(AtomicBool::new(false));
    let mut recording_task: Option<JoinHandle<()>> = None;

    while let Some(msg) = ws.recv().await {

        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    println!("client sent str: {:?}", text);
                    let text_clone = text.clone();
                    let message = text_clone.as_str();
                    match message {
                        "start_recording" => {

                            println!("Start recording from {}", client_addr);
                            recording_flag.store(true, Ordering::SeqCst);

                            let udp_socket = udp_socket.clone();
                            let audio_data = audio_data.clone();
                            let recording_flag = recording_flag.clone();
                            let client_addr = client_addr.clone();

                            if recording_task.is_none() {
                                recording_task = Some(tokio::spawn(async move {
                                    audio_receiver(udp_socket, audio_data, client_addr, recording_flag).await;
                                }));
                            }
                        }
                        "stop_recording" => {
                            recording_flag.store(false, Ordering::SeqCst);

                            if let Some(task) = recording_task.take() {
                                let _ = task.await;
                            }

                            let samples = audio_data.lock().await.clone();
                            println!("Captured {} samples", samples.len());
                            save_wav(audio_path, &samples);
                        }
                        _ => {
                            println!("Unknown message: {}", message);
                        }
                    }

                    if ws.send(Message::Text(format!("You sent: {}", text).into())).await.is_err() {
                        println!("client disconnected");
                        return;
                    }
                }
                Message::Close(c) => {
                    println!("client sent close: {:?}", c);
                    return;
                }
                Message::Binary(_) => { }
                Message::Ping(_) => { }
                Message::Pong(_) => { }
            }
        } else {
            println!("client disconnected");
            return;
        }
    }
}
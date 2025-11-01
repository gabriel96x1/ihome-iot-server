use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::OnceCell;

static UDP_CLIENT: OnceCell<Arc<UdpSocket>> = OnceCell::const_new();

async fn get_udp_client() -> &'static Arc<UdpSocket> {
    UDP_CLIENT
        .get_or_init(|| async {
            let socket = Arc::new(UdpSocket::bind("0.0.0.0:0")
                .await
                .expect("Failed to bind UDP socket"));
            println!("UDP client initialized");
            socket
        })
        .await
}

pub async fn send_udp_message(msg: &str, address: &str) {
    let udp_client = get_udp_client().await;
    udp_client
        .send_to(msg.as_bytes(), address)
        .await
        .expect("Failed to send UDP message");
}
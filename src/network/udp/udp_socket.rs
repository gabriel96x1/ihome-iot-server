use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::OnceCell;

static UDP_SOCKET: OnceCell<Arc<UdpSocket>> = OnceCell::const_new();

pub async fn get_udp_socket() -> Arc<UdpSocket> {
    UDP_SOCKET
        .get_or_init(|| async {
            println!("UDP Server running on port 8080");
            let socket = UdpSocket::bind("0.0.0.0:8080")
                .await
                .expect("Failed to bind UDP socket");
            Arc::new(socket)
        })
        .await
        .clone()
}
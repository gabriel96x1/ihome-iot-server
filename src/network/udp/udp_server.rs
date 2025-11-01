use tokio::net::UdpSocket;

pub async fn run_udp_server() {
    let socket = UdpSocket::bind("0.0.0.0:8080").await;

    match socket {
        Ok(sock) => {
            println!("UDP Server running on port 8080");
            let mut buf = [0u8; 1024];

            loop {
                match sock.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        let msg = String::from_utf8_lossy(&buf[..len]);
                        println!("UDP message received from {}: {}", addr, msg);

                    }
                    Err(e) => {
                        eprintln!("Error receiving UDP packet: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error binding UDP socket: {:?}", e);
        }
    }
}

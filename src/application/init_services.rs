use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use tokio::net::TcpListener;

pub async fn init_services() {
    println!("Launching...");
    println!("{}", BANNER);
    let app = Router::new()
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));

    let listener = TcpListener::bind(addr)
        .await
        .unwrap();

    println!("Server running at: {}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        println!("Server error: {e}");
    }

}

const BANNER: &str = "

██╗██╗░░██╗░█████╗░███╗░░░███╗███████╗  ░██████╗███████╗██████╗░██╗░░░██╗███████╗██████╗░
██║██║░░██║██╔══██╗████╗░████║██╔════╝  ██╔════╝██╔════╝██╔══██╗██║░░░██║██╔════╝██╔══██╗
██║███████║██║░░██║██╔████╔██║█████╗░░  ╚█████╗░█████╗░░██████╔╝╚██╗░██╔╝█████╗░░██████╔╝
██║██╔══██║██║░░██║██║╚██╔╝██║██╔══╝░░  ░╚═══██╗██╔══╝░░██╔══██╗░╚████╔╝░██╔══╝░░██╔══██╗
██║██║░░██║╚█████╔╝██║░╚═╝░██║███████╗  ██████╔╝███████╗██║░░██║░░╚██╔╝░░███████╗██║░░██║
╚═╝╚═╝░░╚═╝░╚════╝░╚═╝░░░░░╚═╝╚══════╝  ╚═════╝░╚══════╝╚═╝░░╚═╝░░░╚═╝░░░╚══════╝╚═╝░░╚═╝

";

async fn health_check() -> &'static str {
    "I'm alive!"
}

fn setup_clients() {
    //setup_mqtt_client();
    //setup_udp_client();
}

fn setup_services() {
    //setup_mqtt_service();
    //setup_ws_service();
    //setup_udp_service();
}
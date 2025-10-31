use infrastructure::network::mqtt::mqtt_client::setup_mqtt_client;
use infrastructure::network::mqtt::mqtt_service::setup_mqtt_service;
use infrastructure::network::websocket::ws_service::setup_ws_service;
use infrastructure::network::udp::udp_client::setup_udp_client;
use infrastructure::network::udp::udp_service::setup_udp_service;

mod domain;
mod infrastructure;
mod application;

#[tokio::main]
async fn main() {
    setup_services();
    setup_clients();
}

fn setup_clients() {
    setup_mqtt_client();
    setup_udp_client();
}

fn setup_services() {
    setup_mqtt_service();
    setup_ws_service();
    setup_udp_service();
}

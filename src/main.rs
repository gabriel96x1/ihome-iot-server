use crate::mqtt::mqtt_client::setup_mqtt_client;
use crate::mqtt::mqtt_service::setup_mqtt_service;
use crate::websocket::ws_service::setup_ws_service;
use crate::udp::udp_client::setup_udp_client;
use crate::udp::udp_service::setup_udp_service;

mod mqtt;
mod websocket;
mod udp;

#[tokio::main]
async fn main() {
    setup_mqtt_service();
    setup_mqtt_client();
    setup_ws_service();
    setup_udp_client();
    setup_udp_service();
}

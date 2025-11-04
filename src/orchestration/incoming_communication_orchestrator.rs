use axum::extract::ws::WebSocket;
use tokio::net::UdpSocket;
use crate::network::tcp::mqtt::mqtt_client::MqttClient;
use crate::network::udp::udp_socket::get_udp_socket;
use crate::process_pipelines::intention_recognition_pipeline::intention_recognition_pipeline;

pub struct IncomingCommunicationOrchestrator<'a> {
    udp_socket: &'a UdpSocket,
    mqtt_client: &'a MqttClient,
}
impl<'a> IncomingCommunicationOrchestrator<'a> {

    pub async fn new() -> Self {
        let udp_socket = get_udp_socket().await;
        let mqtt_client = MqttClient::instance().await;
        Self {
            udp_socket,
            mqtt_client
        }
    }

    pub async fn orchestrate(&mut self, ws: WebSocket, client_addr: std::net::SocketAddr) {
        intention_recognition_pipeline(self.udp_socket, ws, client_addr).await;
    }

}
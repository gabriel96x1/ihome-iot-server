use std::time::Duration;
use crate::network::tcp::mqtt::mqtt_broker::run_mqtt_broker;
use crate::network::tcp::mqtt::mqtt_client::MqttClient;
use crate::network::tcp::tcp_server::run_tcp_server;
use crate::network::udp::udp_client::send_udp_message;

pub async fn init_services() {
    println!("Launching...");
    println!("{}", BANNER);

    tokio::spawn( async {
        run_mqtt_broker().await;
    });

    tokio::spawn(async {
        run_tcp_server().await;
    });

    tokio::spawn(async {
        send_udp_message("holiwis desde el server", "0.0.0.0:8081").await;
    });

    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        MqttClient::subscribe_to_mqtt_topic("test").await;
        let mut rx = MqttClient::subscribe_to_mqtt_events().await;
        loop {
            let event = rx.recv().await.unwrap();
            println!("MQTT event subscriber: {:?}", event);
        }
    });
}

const BANNER: &str = "

██╗██╗░░██╗░█████╗░███╗░░░███╗███████╗  ░██████╗███████╗██████╗░██╗░░░██╗███████╗██████╗░
██║██║░░██║██╔══██╗████╗░████║██╔════╝  ██╔════╝██╔════╝██╔══██╗██║░░░██║██╔════╝██╔══██╗
██║███████║██║░░██║██╔████╔██║█████╗░░  ╚█████╗░█████╗░░██████╔╝╚██╗░██╔╝█████╗░░██████╔╝
██║██╔══██║██║░░██║██║╚██╔╝██║██╔══╝░░  ░╚═══██╗██╔══╝░░██╔══██╗░╚████╔╝░██╔══╝░░██╔══██╗
██║██║░░██║╚█████╔╝██║░╚═╝░██║███████╗  ██████╔╝███████╗██║░░██║░░╚██╔╝░░███████╗██║░░██║
╚═╝╚═╝░░╚═╝░╚════╝░╚═╝░░░░░╚═╝╚══════╝  ╚═════╝░╚══════╝╚═╝░░╚═╝░░░╚═╝░░░╚══════╝╚═╝░░╚═╝

";
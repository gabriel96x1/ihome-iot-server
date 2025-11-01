use crate::network::tcp::mqtt::mqtt_broker::run_mqtt_broker;
use crate::network::tcp::tcp_server::run_tcp_server;
use crate::network::udp::udp_server::run_udp_server;

pub async fn init_services() {
    println!("Launching...");
    println!("{}", BANNER);

    tokio::spawn( async {
        run_mqtt_broker().await;
    });

    tokio::spawn(async {
        run_udp_server().await;
    });

    tokio::spawn( async {
        run_tcp_server().await;
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
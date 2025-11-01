use rumqttd::{Broker, Config};

pub async fn run_mqtt_broker() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let config = config::Config::builder()
        .add_source(config::File::with_name("config/rumqttd.toml"))
        .build()
        .unwrap();

    let rumqttd_config: Config = config.try_deserialize().unwrap();
    let mut broker = Broker::new(rumqttd_config);

    println!("MQTT Broker started at port:1883 for v4.1 port:1884 for v5.1");

    match broker.start() {
        Ok(_) => { }
        Err(e) => {
            println!("MQTT Broker failed to start with error: {}", e);
        }
    }
}
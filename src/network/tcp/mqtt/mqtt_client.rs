use std::sync::Arc;
use rumqttc::{AsyncClient, Event, MqttOptions, QoS};
use tokio::sync::{broadcast, OnceCell};
use tokio::sync::broadcast::Receiver;
use tokio::task;

pub struct MqttClient {
    client: Arc<AsyncClient>,
    event_sender: broadcast::Sender<Event>,
}
static MQTT_CLIENT: OnceCell<MqttClient> = OnceCell::const_new();

impl MqttClient {
    async fn instance() -> &'static Self {
        MQTT_CLIENT
            .get_or_init(|| async {
                let mqttoptions = MqttOptions::new("local-client-server", "0.0.0.0", 1883);
                let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
                let (tx, _rx) = broadcast::channel(32);

                task::spawn({
                    let tx = tx.clone();
                    async move {
                        loop {
                            match eventloop.poll().await {
                                Ok(ev) => {
                                    let _ = tx.send(ev);
                                }
                                Err(e) => {
                                    eprintln!("MQTT eventloop error: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                });
                let atomic_client = Arc::new(client);

                Self { client: atomic_client, event_sender: tx }
            }).await
    }

    pub async fn subscribe_to_mqtt_events() -> Receiver<Event> {
        Self::instance()
            .await
            .event_sender
            .subscribe()
    }

    pub async fn publish_mqtt_message(topic: &str, msg: &str) {
        let mqtt_client = Self::instance().await;
        mqtt_client.client
            .publish(topic, QoS::AtLeastOnce, false, msg)
            .await
            .unwrap();
    }

    pub async fn subscribe_to_mqtt_topic(topic: &str) {
        let mqtt_client = Self::instance().await;
        mqtt_client.client
            .subscribe(topic, QoS::AtMostOnce)
            .await
            .unwrap();
    }
}
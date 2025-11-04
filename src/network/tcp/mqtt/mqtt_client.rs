use std::sync::Arc;
use rumqttc::{AsyncClient, Event, MqttOptions, QoS};
use tokio::sync::{broadcast, OnceCell};
use tokio::sync::broadcast::Receiver;
use tokio::task;

/// A singleton wrapper around an asynchronous MQTT client.
///
/// This structure provides a globally accessible instance of a [`rumqttc::AsyncClient`],
/// along with a [`tokio::sync::broadcast`] channel that distributes all MQTT events
/// (such as incoming messages, connection updates, etc.) to multiple subscribers.
///
/// # Architecture
///
/// - Uses [`OnceCell`] to ensure the client is initialized only once.
/// - Uses [`Arc`] to safely share the MQTT client across concurrent async tasks.
/// - A background task runs the MQTT event loop, forwarding each [`Event`] to all
///   subscribers via a broadcast channel.
///
/// # Example
/// ```rust,no_run
/// use your_crate::MqttClient;
///
/// #[tokio::main]
/// async fn main() {
///     // Subscribe to global MQTT events
///     let mut receiver = MqttClient::subscribe_to_mqtt_events().await;
///
///     // Spawn a background task to listen for events
///     tokio::spawn(async move {
///         while let Ok(event) = receiver.recv().await {
///             println!("Received MQTT event: {:?}", event);
///         }
///     });
///
///     // Publish a message
///     MqttClient::publish_mqtt_message("test/topic", "Hello from Rust!").await;
///
///     // Subscribe to a topic
///     MqttClient::subscribe_to_mqtt_topic("test/topic").await;
/// }
/// ```
pub struct MqttClient {
    /// Shared asynchronous MQTT client.
    client: Arc<AsyncClient>,
    /// Broadcast channel used to send MQTT events to all subscribers.
    event_sender: broadcast::Sender<Event>,
}

/// Global singleton instance of the MQTT client.
///
/// Lazily initialized on first use, ensuring that only one
/// MQTT client exists throughout the entire process.
static MQTT_CLIENT: OnceCell<MqttClient> = OnceCell::const_new();

impl MqttClient {
    /// Returns a reference to the global MQTT client instance.
    ///
    /// If the client has not been initialized yet, it will be created automatically:
    /// - Connects to the MQTT broker at `0.0.0.0:1883` using client ID `local-client-server`.
    /// - Starts an MQTT event loop in a background task.
    /// - Sets up a broadcast channel to forward all MQTT events.
    pub async fn instance() -> &'static Self {
        MQTT_CLIENT
            .get_or_init(|| async {
                // Basic client configuration
                let mqttoptions = MqttOptions::new("local-client-server", "0.0.0.0", 1883);
                let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
                let (tx, _rx) = broadcast::channel(32);

                // Spawn a background task to handle MQTT events
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

    /// Returns a new [`Receiver`] to listen to all MQTT events emitted by the client.
    ///
    /// Each call returns a unique receiver, allowing multiple consumers to listen to
    /// the same event stream independently.
    ///
    /// # Example
    /// ```rust,no_run
    /// let mut rx = MqttClient::subscribe_to_mqtt_events().await;
    /// while let Ok(event) = rx.recv().await {
    ///     println!("{:?}", event);
    /// }
    /// ```
    pub async fn subscribe_to_mqtt_events() -> Receiver<Event> {
        Self::instance()
            .await
            .event_sender
            .subscribe()
    }

    /// Publishes a message to the specified topic.
    ///
    /// # Parameters
    /// - `topic`: The MQTT topic name.
    /// - `msg`: The message payload.
    ///
    /// Uses [`QoS::AtLeastOnce`] to ensure at least one delivery,
    /// though duplicates may occur if the connection drops.
    ///
    /// # Example
    /// ```rust,no_run
    /// MqttClient::publish_mqtt_message("test/topic", "hello world").await;
    /// ```
    pub async fn publish_mqtt_message(topic: &str, msg: &str) {
        let mqtt_client = Self::instance().await;
        mqtt_client.client
            .publish(topic, QoS::AtLeastOnce, false, msg)
            .await
            .unwrap();
    }

    /// Subscribes to a given MQTT topic using [`QoS::AtMostOnce`].
    ///
    /// # Parameters
    /// - `topic`: The topic to subscribe to.
    ///
    /// This method only registers the subscription.
    /// To receive messages, use [`subscribe_to_mqtt_events()`] and listen for
    /// `Event::Incoming::Publish` events.
    ///
    /// # Example
    /// ```rust,no_run
    /// MqttClient::subscribe_to_mqtt_topic("sensor/temperature").await;
    /// ```
    pub async fn subscribe_to_mqtt_topic(topic: &str) {
        let mqtt_client = Self::instance().await;
        mqtt_client.client
            .subscribe(topic, QoS::AtMostOnce)
            .await
            .unwrap();
    }
}

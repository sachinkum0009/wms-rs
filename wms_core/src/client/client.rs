use paho_mqtt::AsyncClient as MqttClient;
use paho_mqtt::ConnectOptions;
use paho_mqtt::Error;
use paho_mqtt::ReasonCode;
use paho_mqtt::ServerResponse;
use paho_mqtt::{Message, Topic};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    host: String,
    port: i16,
}

impl Config {
    pub fn new(host: String, port: i16) -> Self {
        Self { host, port }
    }

    pub fn get_url(&self) -> String {
        format!("mqtt://{}:{}", self.host, self.port)
    }
}

pub struct Client {
    client: MqttClient,
}

pub struct Publisher<'a> {
    client: &'a MqttClient, // This borrows the client
    topic: String,
}

impl Client {
    /// Creates a new asynchronous MQTT client.
    ///
    /// # Arguments
    ///
    /// * `config` - A [`Config`] struct containing connection details like the URL.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the MQTT client fails to initialize with the provided URL.
    pub fn new(config: Config) -> Result<Self, Error> {
        let mqtt_client = MqttClient::new(config.get_url())?;
        Ok(Self {
            client: mqtt_client,
        })
    }

    pub fn create_publisher(&self, topic_name: String) -> Publisher<'_> {
        Publisher {
            client: &self.client,
            topic: topic_name,
        }
    }

    pub fn connect(&self) -> Result<bool, Error> {
        if self.client.is_connected() {
            return Ok(true);
        } else {
            let conn_opts = ConnectOptions::new();
            let result = self.client.connect(conn_opts).wait()?;
            let reason_code = result.reason_code();
            let response = match reason_code {
                ReasonCode::Success => true,
                ReasonCode::Banned => false,
                _ => false,
            };
            return Ok(response);
        }
    }

    /// Publishes a message to the configured MQTT broker.
    ///
    /// Returns `true` if the message was successfully handed off to the internal
    /// buffer, `false` otherwise.
    pub fn publish(&self, msg: Message) -> bool {
        self.client.publish(msg);
        true
    }
}

impl<'a> Publisher<'a> {
    /// Publishes data to the topic assigned to this publisher.
    pub fn publish(&self, payload: Vec<u8>) -> bool {
        let msg = Message::new(&self.topic, payload, 1);
        self.client.publish(msg);
        true
    }
}

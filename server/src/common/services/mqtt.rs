use paho_mqtt::{AsyncClient, Message};
use serde_json::json;

#[derive(Clone)]
pub struct MqttService {
    pub client: AsyncClient,
}

impl MqttService {
    pub fn new(client: AsyncClient) -> MqttService {
        MqttService { client }
    }

    pub fn update_language(self, device_id: &String, from: &String, to: &String) {
        let message = json!({"cmd": "setLanguage","up": from,"down": to});

        self.send_message(&message.to_string(), device_id)
    }

    pub fn bind_success(self, device_id: &String) {
        let message = json!({"cmd": "bindSuccess"});
        self.send_message(&message.to_string(), device_id)
    }

    fn send_message(self, message: &String, device_id: &String) {
        let product_key = std::env::var("ALICLOUD_PRODUCT_KEY").unwrap();
        let msg = Message::new(
            format!("/{}/{}/get", product_key, device_id),
            message.to_owned(),
            paho_mqtt::QOS_2,
        );

        self.client.publish(msg);
    }
}

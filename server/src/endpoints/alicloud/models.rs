use crate::endpoints::errors::AppError;
use hmac::digest::MacError;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::fmt::{Display, Formatter};
use AppError::AuthError;

#[derive(Deserialize)]
pub struct AliDeviceRegistration {
    #[serde(rename = "productKey")]
    pub product_key: String,
    #[serde(rename = "deviceName")]
    pub device_name: String,
    pub signmethod: String,
    pub sign: String,
    pub version: String,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub timestamp: String,
    pub resources: String,
}

impl Display for AliDeviceRegistration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "product_key: [{}], device_name: [{}], signmethod: [{}], sign: [{}], version: [{}], client_id: [{}], timestamp: [{}], resources: [{}], ",
               self.product_key, self.device_name, self.signmethod, self.sign, self.version, self.client_id, self.timestamp, self.resources)
    }
}

impl AliDeviceRegistration {
    fn create_hmac_source(&self) -> String {
        let timestamp = String::from("2524608000000");
        format!(
            "clientId{}.{}deviceName{}productKey{}timestamp{}",
            self.product_key, self.device_name, self.device_name, self.product_key, timestamp
        )
    }

    pub fn verify_hmac(&self, key: &[u8]) -> Result<(), AppError> {
        let hmac_source = self.create_hmac_source();
        let sign =
            hex::decode(&self.sign).map_err(|_| AppError::Generic("Cannot verify HMAC".into()))?;
        verify_hmac(hmac_source, key, sign.as_slice()).map_err(|e| {
            log::info!("{}", e);
            AuthError
        })
    }
}

#[derive(Serialize)]
pub struct AliDeviceNameResponse {
    #[serde(rename = "iotId")]
    pub iot_id: String,
    #[serde(rename = "iotToken")]
    pub iot_token: String,
    pub resources: Resources,
}

impl AliDeviceNameResponse {
    pub fn new(iot_id: String, iot_token: String) -> Self {
        AliDeviceNameResponse {
            iot_id,
            iot_token,
            resources: Resources {
                mqtt: MqttResource {
                    host: String::from("iot-auth.cn-shanghai.aliyuncs.com"),
                    port: 8883,
                },
            },
        }
    }
}

#[derive(Serialize)]
pub struct Resources {
    pub mqtt: MqttResource,
}

#[derive(Serialize)]
pub struct MqttResource {
    pub host: String,
    pub port: u16,
}

fn verify_hmac(source: String, key: &[u8], sign: &[u8]) -> Result<(), MacError> {
    type HmacSha1 = Hmac<Sha1>;
    let mut mac = HmacSha1::new_from_slice(key).expect("HMAC Key should be valid");
    mac.update(source.as_bytes());

    mac.verify_slice(sign)
}

#[cfg(test)]
mod tests {
    use super::verify_hmac;

    #[test]
    fn test_calculate_hmac() {
        let hmac_source = "clientIdabcdefghijk.22222222222222222222222222222222deviceName22222222222222222222222222222222productKeyabcdefghijktimestamp2524608000000";
        let key = "11111111111111111111111111111111".as_bytes();
        let sign = hex::decode("52a4f5b3a6f388b314ee1af6b386f983d0d020ac").unwrap();
        let hmac_result = verify_hmac(String::from(hmac_source), key, sign.as_slice());
        assert!(hmac_result.is_ok());
    }
}

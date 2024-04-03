use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct DeviceRegistrationRequest {
    pub mac: String,
    verify_code: String,
}

impl Display for DeviceRegistrationRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mac: [{}], verify_code: [{}]",
            self.mac, self.verify_code
        )
    }
}

#[derive(Serialize)]
pub struct DeviceRegistrationResponse {
    pub device_id: String,
    pub device_secret: String,
}

impl DeviceRegistrationResponse {
    pub fn new(device_id: String, device_secret: String) -> Self {
        DeviceRegistrationResponse {
            device_id,
            device_secret,
        }
    }
}

#[derive(Deserialize)]
pub struct NetworkRequest {
    pub device_id: String,
    pub ssid: String,
}

impl Display for NetworkRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "device_id: [{}], ssid: [{}]", self.device_id, self.ssid)
    }
}

#[derive(Deserialize)]
pub struct VersionRequest {
    pub device_id: String,
    pub version: String,
}

impl Display for VersionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "device_id: [{}], version: [{}]",
            self.device_id, self.version
        )
    }
}

#[derive(Deserialize)]
pub struct VerifyCodeRequest {
    pub device_id: String,
}

impl Display for VerifyCodeRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "device_id: [{}]", self.device_id)
    }
}

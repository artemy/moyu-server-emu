use crate::endpoints::app::common::{AppAuth, Auth};
use crate::endpoints::language::Language;
use chrono::serde::ts_seconds_option;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct DeviceIdRequest {
    #[serde(flatten)]
    auth: AppAuth,
    #[serde(rename = "verifyCode")]
    pub verify_code: String,
}

impl Display for DeviceIdRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "verify_code: [{}], {}", self.verify_code, self.auth)
    }
}

impl Auth for DeviceIdRequest {
    fn auth(&self) -> &str {
        self.auth.auth.as_str()
    }

    fn service_token(&self) -> &str {
        self.auth.service_token.as_str()
    }

    fn auth_value(&self) -> String {
        format!(
            "{}{}{}",
            self.verify_code, self.auth.app_id, self.auth.service_token
        )
    }
}

#[derive(Serialize)]
pub struct DeviceIdResponse {
    device_id: String,
}

impl DeviceIdResponse {
    pub fn new(device_id: String) -> Self {
        DeviceIdResponse { device_id }
    }
}

#[derive(Deserialize)]
pub struct VersionRequest {
    #[serde(flatten)]
    auth: AppAuth,
    device_id: String,
}

impl Display for VersionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "device_id: [{}], {}", self.device_id, self.auth)
    }
}

impl Auth for VersionRequest {
    fn auth(&self) -> &str {
        self.auth.auth.as_str()
    }

    fn service_token(&self) -> &str {
        self.auth.service_token.as_str()
    }

    fn auth_value(&self) -> String {
        format!(
            "{}{}{}",
            self.device_id, self.auth.app_id, self.auth.service_token
        )
    }
}

#[derive(Deserialize)]
pub struct LanguageListRequest {
    #[serde(flatten)]
    auth: AppAuth,
    get_type: i8,
}

impl Display for LanguageListRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "get_type: [{}], {}", self.get_type, self.auth)
    }
}

impl Auth for LanguageListRequest {
    fn auth(&self) -> &str {
        self.auth.auth.as_str()
    }

    fn service_token(&self) -> &str {
        self.auth.service_token.as_str()
    }
    fn auth_value(&self) -> String {
        format!(
            "{}{}{}",
            self.get_type, self.auth.app_id, self.auth.service_token
        )
    }
}

#[derive(Serialize)]
pub struct LanguageItem {
    name: String,
    key: String,
}
impl From<Language> for LanguageItem {
    fn from(value: Language) -> Self {
        LanguageItem {
            name: value.name,
            key: value.iso_code,
        }
    }
}

#[derive(Deserialize)]
pub struct SetLanguageRequest {
    #[serde(flatten)]
    auth: AppAuth,
    pub id: i64,
    pub from: String,
    pub to: String,
}

impl Display for SetLanguageRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: [{}], from: [{}], to: [{}], {}",
            self.id, self.from, self.to, self.auth
        )
    }
}

impl Auth for SetLanguageRequest {
    fn auth(&self) -> &str {
        self.auth.auth.as_str()
    }
    fn service_token(&self) -> &str {
        self.auth.service_token.as_str()
    }
    fn auth_value(&self) -> String {
        format!("{}{}{}", self.from, self.id, self.to)
    }
}

#[derive(Deserialize)]
pub struct SoundNetworkRequest {
    auth: String,
    pub password: String,
    pub ssid: String,
    wx_open_id: String,
}

impl Display for SoundNetworkRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "auth: [{}], ssid: [{}], password: [{}], wx_open_id: [{}], ",
            self.auth, self.ssid, self.password, self.wx_open_id
        )
    }
}

impl Auth for SoundNetworkRequest {
    fn auth(&self) -> &str {
        self.auth.as_str()
    }
    fn service_token(&self) -> &str {
        self.wx_open_id.as_str()
    }
    fn auth_value(&self) -> String {
        format!("{}{}{}", self.password, self.ssid, self.wx_open_id)
    }
}

#[derive(Serialize)]
pub struct VersionInfo {
    pub id: u32,
    pub filesize: String,
    pub version: String,
}

pub type PushUpdateVersionRequest = VersionRequest;

pub type ModelsListRequest = AppAuth;

#[derive(Serialize)]
pub struct ModelInfo {
    pub sort: i32,
    pub status: i32,
    #[serde(with = "ts_seconds_option")]
    pub create_time: Option<chrono::DateTime<Utc>>,
    pub id: String,
    pub name: String,
}

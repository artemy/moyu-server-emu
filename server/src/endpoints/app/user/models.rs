use crate::endpoints::app::common::{AppAuth, Auth};
use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::AuthError;
use chrono::serde::ts_seconds_option;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;
use serde_with::serde_as;
use serde_with::BoolFromInt;
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct UserIdRequest {
    auth: String,
    pub imei: String,
    pub token: String,
}

impl Display for UserIdRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "auth: [{}], imei: [{}], token: [{}]",
            self.auth, self.imei, self.token
        )
    }
}

impl Auth for UserIdRequest {
    fn auth(&self) -> &str {
        self.auth.as_str()
    }

    fn service_token(&self) -> &str {
        "NOT_APPLICABLE"
    }

    fn auth_value(&self) -> String {
        format!("{}{}", self.imei, self.token)
    }
}

#[derive(Deserialize)]
pub struct HistoryRequest {
    #[serde(flatten)]
    auth: AppAuth,
    pub page: u64,
    pub size: u64,
}

impl Display for HistoryRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "page: [{}], size: [{}], {}",
            self.page, self.size, self.auth
        )
    }
}

impl Auth for HistoryRequest {
    fn auth(&self) -> &str {
        self.auth.auth.as_str()
    }

    fn service_token(&self) -> &str {
        self.auth.service_token.as_str()
    }

    fn auth_value(&self) -> String {
        format!(
            "{}{}{}{}",
            self.auth.app_id, self.page, self.auth.service_token, self.size
        )
    }
}

#[derive(Deserialize)]
pub struct BindRequest {
    auth: String,
    pub device_id: String,
    #[serde(rename = "serviceToken")]
    service_token: String,
}

impl Display for BindRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "auth: [{}], device_id: [{}], service_token: [{}]",
            self.auth, self.device_id, self.service_token
        )
    }
}

impl Auth for BindRequest {
    fn auth(&self) -> &str {
        self.auth.as_str()
    }

    fn service_token(&self) -> &str {
        self.service_token.as_str()
    }
    fn auth_value(&self) -> String {
        format!("{}{}", self.device_id, self.service_token)
    }
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    auth: String,
    #[serde(rename = "serviceToken")]
    service_token: String,
}

impl Display for LogoutRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "auth: [{}], service_token: [{}]",
            self.auth, self.service_token
        )
    }
}

impl Auth for LogoutRequest {
    fn auth(&self) -> &str {
        self.auth.as_str()
    }

    fn service_token(&self) -> &str {
        self.service_token.as_str()
    }

    fn auth_value(&self) -> String {
        self.service_token.to_string()
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "serviceToken")]
    pub service_token: String,
}

impl UserResponse {
    pub fn new(user_id: String, service_token: String) -> Self {
        UserResponse {
            user_id,
            service_token,
        }
    }
}

#[derive(Serialize)]
pub struct ChatInfo {
    #[serde(with = "ts_seconds_option")]
    pub create_time: Option<chrono::DateTime<Utc>>,
    pub id: String,
    pub device_id: String,
    pub wx_open_id: String,
    pub language_from: String,
    pub language_to: String,
    pub from_text: String,
    pub to_text: String,
    // 2 = self, 1 = other
    pub is_myself: ChatParticipant,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ChatParticipant {
    Me = 2,
    Other = 1,
}

pub type DeviceInfoRequest = AppAuth;

#[serde_as]
#[derive(Serialize)]
pub struct DeviceInfo {
    pub id: String,
    #[serde(rename = "status")]
    #[serde_as(as = "BoolFromInt")]
    pub online: bool,
    pub setting: DeviceSetting,
    pub mac: String,
    pub ssid: String,
    pub device_id: String,
    pub version: String,
}

#[derive(Serialize)]
pub struct DeviceSetting {
    #[serde(with = "ts_seconds_option")]
    pub update_time: Option<chrono::DateTime<Utc>>,
    pub language_from: String,
    pub language_to: String,
}

pub type UnbindRequest = BindRequest;

pub trait ValidateUserRegistrationToken {
    fn validate_token(&self) -> Result<(), AppError>;
}

impl ValidateUserRegistrationToken for UserIdRequest {
    fn validate_token(&self) -> Result<(), AppError> {
        let computed = compute_token(&self.imei);
        if !self.token.eq(&computed) {
            log::warn!(
                "Invalid token: [{}], calculated: [{}]",
                &self.token,
                &computed
            );
            return Err(AuthError);
        }
        Ok(())
    }
}

fn compute_token(imei: &String) -> String {
    let source = format!("{}{}", &imei, "moyu");
    let first_pass = md5::compute(source);
    log::debug!("Source encoded: {:x}", &first_pass);
    let digest = md5::compute(format!("{:x}", first_pass));

    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use super::compute_token;

    #[test]
    fn test_compute_token() {
        let imei = "098765432109876";

        let expected = "36f2a1b7642d6c4da842f603be690d05";

        assert_eq!(expected, compute_token(&imei.into()));
    }
}

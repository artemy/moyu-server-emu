use crate::endpoints::app::common::Auth;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct AppVersionRequest {
    auth: String,
    platform: String,
    version: String,
}

impl Display for AppVersionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "auth: [{}], platform: [{}], version: [{}]",
            self.auth, self.platform, self.version
        )
    }
}

impl Auth for AppVersionRequest {
    fn auth(&self) -> &str {
        self.auth.as_str()
    }

    fn service_token(&self) -> &str {
        "NOT_APPLICABLE"
    }
    fn auth_value(&self) -> String {
        format!("{}{}", self.platform, self.version)
    }
}

#[derive(Serialize)]
pub struct UpdateInfo {
    pub description: String,
    pub size: String,
    pub url: String,
    pub version: String,
    pub version_name: String,
}

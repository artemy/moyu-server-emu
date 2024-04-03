use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize)]
pub struct QaRequest {
    pub device_id: String,
    remaining_power: String,
    session_id: String,
}

impl Display for QaRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "device_id: [{}], remaining_power: [{}], session_id: [{}]",
            self.device_id, self.remaining_power, self.session_id
        )
    }
}

#[derive(Deserialize)]
pub struct TranslateRequest {
    pub device_id: String,
    pub from: String,
    pub to: String,
}

impl Display for TranslateRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "device_id: [{}], from: [{}], to: [{}]",
            self.device_id, self.from, self.to
        )
    }
}

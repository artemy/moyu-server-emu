use crate::endpoints::models::BaseResult;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::fmt::{Display, Formatter};

pub enum AppError {
    AuthError,
    UserNotFound,
    DeviceNotFound,
    DatabaseError(String),
    IOError(String),
    Generic(String),
    OpenAIError(String),
}

impl AppError {
    fn to_error_code(&self) -> i32 {
        match self {
            AppError::AuthError => 400,
            AppError::UserNotFound => 400,
            AppError::DeviceNotFound => 400,
            AppError::DatabaseError(_) => 500,
            AppError::IOError(_) => 500,
            AppError::Generic(_) => 500,
            AppError::OpenAIError(_) => 500,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let result: BaseResult<()> = BaseResult {
            code: self.to_error_code(),
            data: None,
            info: Some(self.to_string()),
        };
        Json(result).into_response()
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error = match self {
            AppError::AuthError => "Auth error",
            AppError::UserNotFound => "User not found",
            AppError::DeviceNotFound => "Device not found",
            AppError::DatabaseError(err) => err,
            AppError::Generic(err) => err,
            AppError::IOError(err) => err,
            AppError::OpenAIError(err) => err,
        };
        write!(f, "{}", error)
    }
}

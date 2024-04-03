use crate::endpoints::errors::AppError;
use crate::endpoints::errors::AppError::AuthError;
use axum_extra::extract::CookieJar;

pub trait ServiceTokenCookie {
    fn validate_service_token(&self) -> Result<String, AppError>;
}

impl ServiceTokenCookie for CookieJar {
    fn validate_service_token(&self) -> Result<String, AppError> {
        let service_token = self
            .get("serviceToken")
            .map(|cookie| cookie.value().to_owned())
            .ok_or(AuthError)?;
        if service_token.is_empty() {
            return Err(AuthError);
        }
        Ok(service_token)
    }
}

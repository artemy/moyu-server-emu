use crate::endpoints::app::common::Auth;
use crate::endpoints::app::version::models::{AppVersionRequest, UpdateInfo};
use crate::endpoints::errors::AppError;
use crate::endpoints::models::BaseResult;
use axum::extract::Query;
use axum::Json;

pub async fn android(
    Query(payload): Query<AppVersionRequest>,
) -> Result<Json<BaseResult<UpdateInfo>>, AppError> {
    log::info!("New android app version request: {}", payload);

    payload.validate_auth_old()?;
    let response = BaseResult::new(UpdateInfo {
        description: String::from("New version"),
        version: String::from("2023"),
        version_name: String::from("123"),
        url: String::from(""),
        size: String::from("1M"),
    });

    Ok(Json(response))
}

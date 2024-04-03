use crate::common::services::device::DeviceService;
use crate::endpoints::errors::AppError;
use crate::endpoints::models::{BaseResult, UrlResponse};
use crate::endpoints::outer::device::middlewares::PlainJson;
use crate::endpoints::outer::device::misc;
use crate::endpoints::outer::device::models::{
    DeviceRegistrationRequest, DeviceRegistrationResponse, NetworkRequest, VerifyCodeRequest,
    VersionRequest,
};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;

pub async fn get_device_id_by_mac(
    State(device_service): State<DeviceService>,
    PlainJson(payload): PlainJson<DeviceRegistrationRequest>,
) -> Result<Json<BaseResult<DeviceRegistrationResponse>>, AppError> {
    log::info!("New device registration: {}", payload);

    let device_data = device_service
        .get_or_create_device_data(&payload.mac)
        .await?;
    let response = BaseResult::new(DeviceRegistrationResponse::new(
        device_data.device_id,
        device_data.device_secret,
    ));
    Ok(Json(response))
}

pub async fn network(
    State(device_service): State<DeviceService>,
    Query(payload): Query<NetworkRequest>,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("New device registration: {}", payload);

    device_service
        .update_ssid(&payload.device_id, &payload.ssid)
        .await?;
    Ok(Json(BaseResult::<()>::none()))
}

pub async fn upload_version(
    State(device_service): State<DeviceService>,
    Query(payload): Query<VersionRequest>,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("Device reports version registration: {}", payload);

    device_service
        .update_version(&payload.device_id, &payload.version)
        .await?;
    Ok(Json(BaseResult::<()>::none()))
}

pub async fn verify_code(
    State(AppState {
        device_service,
        openai,
        file_service,
        ..
    }): State<AppState>,
    PlainJson(payload): PlainJson<VerifyCodeRequest>,
) -> Result<Json<BaseResult<UrlResponse>>, AppError> {
    log::info!("Verify code request: {}", payload);

    let verify_code = device_service
        .generate_and_return_verify_code(&payload.device_id)
        .await?;
    let verify_code = misc::add_commas(verify_code);

    log::info!(
        "Generated verify code: [{}] for device_id: [{}]",
        verify_code,
        payload.device_id
    );
    let response = openai
        .text_to_speech(&verify_code, Some(1.0))
        .await
        .map_err(|e| AppError::OpenAIError(e.to_string()))?;
    let filename = format!("verify-code-{}.mp3", payload.device_id);

    file_service
        .save_file_and_return_url(&filename, &response)
        .map(|url| Json(BaseResult::new(UrlResponse::new(url))))
}

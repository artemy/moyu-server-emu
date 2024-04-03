use crate::common::services::device::DeviceService;
use crate::endpoints::alicloud::models::{AliDeviceNameResponse, AliDeviceRegistration};
use crate::endpoints::errors::AppError;
use crate::endpoints::models::BaseResult;
use axum::extract::State;
use axum::{Form, Json};

pub async fn ali_cloud(
    State(device_service): State<DeviceService>,
    Form(payload): Form<AliDeviceRegistration>,
) -> Result<Json<BaseResult<AliDeviceNameResponse>>, AppError> {
    log::info!("New device registration: {}", payload);

    let device_data = device_service
        .get_device_data_by_device_id(&payload.device_name)
        .await?;
    payload.verify_hmac(device_data.device_secret.as_bytes())?;

    let response = BaseResult {
        code: 200,
        data: Some(AliDeviceNameResponse::new(
            device_data.device_id,
            device_data.device_secret,
        )),
        info: None,
    };
    Ok(Json(response))
}

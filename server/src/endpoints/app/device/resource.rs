use crate::common::services::user::UserService;
use crate::endpoints::app::common::Auth;
use crate::endpoints::app::device::misc::ServiceTokenCookie;
use crate::endpoints::app::device::models::{
    DeviceIdRequest, DeviceIdResponse, LanguageItem, LanguageListRequest, ModelInfo,
    ModelsListRequest, PushUpdateVersionRequest, SetLanguageRequest, SoundNetworkRequest,
    VersionInfo, VersionRequest,
};
use crate::endpoints::app::device::soundwave::credentials_to_wave;
use crate::endpoints::common::DATE_FORMAT;
use crate::endpoints::errors::AppError;
use crate::endpoints::language::{supported_languages, Language};
use crate::endpoints::models::BaseResult;
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use axum_extra::extract::CookieJar;
use AppError::DeviceNotFound;

pub async fn get_device_id_by_verify_code(
    State(AppState {
        user_service,
        device_service,
        ..
    }): State<AppState>,
    Query(payload): Query<DeviceIdRequest>,
    cookies: CookieJar,
) -> Result<Json<BaseResult<DeviceIdResponse>>, AppError> {
    log::info!("New get device by verify code request: {}", payload);

    payload.validate_auth()?;
    let service_token = cookies.validate_service_token()?;

    user_service
        .get_user_data_by_service_token(&service_token)
        .await?;

    let device_data = device_service
        .get_device_data_by_verify_code(&payload.verify_code)
        .await?;
    let response = BaseResult::new(DeviceIdResponse::new(device_data.device_id));
    Ok(Json(response))
}

pub async fn get_latest_version(
    State(user_service): State<UserService>,
    Query(payload): Query<VersionRequest>,
    cookies: CookieJar,
) -> Result<Json<BaseResult<VersionInfo>>, AppError> {
    log::info!("New version request: {}", payload);

    payload.validate_auth()?;
    let service_token = cookies.validate_service_token()?;

    user_service
        .get_user_data_by_service_token(&service_token)
        .await?;

    let response = BaseResult::new(VersionInfo {
        id: 1,
        filesize: String::from("4"),
        version: String::from("0.4.3"),
    });

    Ok(Json(response))
}

pub async fn push_update_version(
    State(user_service): State<UserService>,
    Query(payload): Query<PushUpdateVersionRequest>,
    cookies: CookieJar,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("New push update version request: {}", payload);

    payload.validate_auth()?;
    let service_token = cookies.validate_service_token()?;

    user_service
        .get_user_data_by_service_token(&service_token)
        .await?;

    let response: BaseResult<()> = BaseResult {
        code: 1,
        data: None,
        info: Some(String::from("Update pushed")),
    };

    Ok(Json(response))
}

pub async fn get_language_list(
    State(user_service): State<UserService>,
    Query(payload): Query<LanguageListRequest>,
    cookies: CookieJar,
) -> Result<Json<BaseResult<Vec<LanguageItem>>>, AppError> {
    log::info!("New get language list request: {}", payload);

    payload.validate_auth()?;
    let service_token = cookies.validate_service_token()?;

    user_service
        .get_user_data_by_service_token(&service_token)
        .await?;
    let languages: Vec<LanguageItem> = supported_languages()
        .into_iter()
        .map(|lang| lang.into())
        .collect();

    let response = BaseResult::new(languages);

    Ok(Json(response))
}

pub async fn set_language(
    State(AppState {
        user_service,
        device_service,
        mqtt,
        ..
    }): State<AppState>,
    Query(payload): Query<SetLanguageRequest>,
    cookies: CookieJar,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("New set language request: {}", payload);

    payload.validate_auth()?;
    let service_token = cookies.validate_service_token()?;

    let user_data = user_service
        .get_user_data_by_service_token(&service_token)
        .await?;

    let device_data = device_service
        .get_device_data_by_user_id(user_data.id)
        .await?
        .ok_or(DeviceNotFound)?;
    if device_data.id != payload.id {
        return Err(DeviceNotFound);
    }

    let from = Language::from_iso_code(&payload.from)?;
    let to = Language::from_iso_code(&payload.to)?;

    device_service
        .update_language(&device_data.device_id, &to.iso_code, &from.iso_code)
        .await?;

    mqtt.update_language(&device_data.device_id, &from.device_code, &to.device_code);

    let response = BaseResult::<()>::none();
    Ok(Json(response))
}

pub async fn get_models_list(
    Query(payload): Query<ModelsListRequest>,
) -> Result<Json<BaseResult<Vec<ModelInfo>>>, AppError> {
    log::info!("New get models request: {}", payload);

    let response = BaseResult::new(vec![]);

    Ok(Json(response))
}

pub async fn sound_network(
    State(AppState {
        user_service,
        file_service,
        ..
    }): State<AppState>,
    Query(payload): Query<SoundNetworkRequest>,
) -> Result<Json<BaseResult<String>>, AppError> {
    log::info!("New sound network request: {}", payload);

    payload.validate_auth()?;
    user_service
        .get_user_data_by_service_token(payload.service_token())
        .await?;

    let bytes = credentials_to_wave(&payload.ssid, &payload.password);
    let timestamp = chrono::Utc::now().format(DATE_FORMAT);
    let filename = format!("soundwave-{}.wav", timestamp);

    file_service
        .save_file_and_return_url(&filename, &bytes)
        .map(|url| Json(BaseResult::new(url)))
}

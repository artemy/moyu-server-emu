use crate::common::services::user::UserService;
use crate::endpoints::app::common::Auth;
use crate::endpoints::app::user::models::{
    BindRequest, ChatInfo, ChatParticipant, DeviceInfo, DeviceInfoRequest, DeviceSetting,
    HistoryRequest, LogoutRequest, UnbindRequest, UserIdRequest, UserResponse,
    ValidateUserRegistrationToken,
};
use crate::endpoints::errors::AppError;
use crate::endpoints::models::{BaseList, BaseResult};
use crate::AppState;
use axum::extract::{Query, State};
use axum::{Form, Json};

pub async fn get_user_id_by_imei(
    State(user_service): State<UserService>,
    Form(payload): Form<UserIdRequest>,
) -> Result<Json<BaseResult<UserResponse>>, AppError> {
    log::info!("New user id request: {}", payload);

    payload.validate_auth()?;
    payload.validate_token()?;

    let user = user_service.get_or_create_user(&payload.imei).await?;
    let response = BaseResult::new(UserResponse::new(user.user_id, user.service_token));
    Ok(Json(response))
}

pub async fn get_history_by_user_id(
    State(AppState {
        user_service,
        history_service,
        ..
    }): State<AppState>,
    Form(payload): Form<HistoryRequest>,
) -> Result<Json<BaseResult<BaseList<ChatInfo>>>, AppError> {
    log::info!("New history request: {}", payload);

    payload.validate_auth()?;
    let user_data = user_service
        .get_user_data_by_service_token(payload.service_token())
        .await?;

    let records = history_service
        .get_chat_history_by_user_id(user_data.id, payload.page, payload.size)
        .await?;

    let response = BaseResult::new(BaseList {
        count: records.count,
        list: records
            .list
            .iter()
            .map(|r| ChatInfo {
                create_time: Some(r.created.to_utc()),
                id: r.id.to_string(),
                device_id: r.device_id.to_string(),
                wx_open_id: "".to_string(), // dummy
                language_from: r.from.clone(),
                language_to: r.to.clone(),
                from_text: r.from_text.clone(),
                to_text: r.to_text.clone(),
                is_myself: if r.is_myself {
                    ChatParticipant::Me
                } else {
                    ChatParticipant::Other
                },
            })
            .collect(),
        more: records.more,
    });

    Ok(Json(response))
}

pub async fn get_device_info_by_user_id(
    State(AppState {
        user_service,
        device_service,
        ..
    }): State<AppState>,
    Form(payload): Form<DeviceInfoRequest>,
) -> Result<Json<BaseResult<DeviceInfo>>, AppError> {
    log::info!("New device info request: {}", payload);

    payload.validate_auth()?;

    let user_data = user_service
        .get_user_data_by_service_token(&payload.service_token)
        .await?;
    let device_data = device_service
        .get_device_data_by_user_id(user_data.id)
        .await?
        .map(|device_data| {
            DeviceInfo {
                id: device_data.id.to_string(),
                online: true, // assuming device is always online
                setting: DeviceSetting {
                    update_time: Some(device_data.updated.to_utc()),
                    language_from: device_data.language_from,
                    language_to: device_data.language_to,
                },
                mac: device_data.mac,
                ssid: device_data.ssid.unwrap_or_default(),
                device_id: device_data.device_id,
                version: device_data.version.unwrap_or_default(),
            }
        })
        .map_or_else(
            || BaseResult {
                code: 0,
                data: None,
                info: Some("No device".into()),
            },
            BaseResult::new,
        );

    Ok(Json(device_data))
}

pub async fn bind(
    State(AppState {
        user_service,
        device_service,
        mqtt,
        ..
    }): State<AppState>,
    Query(payload): Query<BindRequest>,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("New bind request: {}", payload);

    payload.validate_auth()?;
    let user_data = user_service
        .get_user_data_by_service_token(payload.service_token())
        .await?;

    device_service
        .bind_device_to_user(user_data.id, &payload.device_id)
        .await?;

    mqtt.bind_success(&payload.device_id);

    let response: BaseResult<()> = BaseResult {
        code: 0,
        data: None,
        info: Some(String::from("Bind success")),
    };

    Ok(Json(response))
}

pub async fn unbind(
    State(AppState {
        user_service,
        device_service,
        ..
    }): State<AppState>,
    Query(payload): Query<UnbindRequest>,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("New unbind request: {}", payload);

    payload.validate_auth()?;
    let user_data = user_service
        .get_user_data_by_service_token(payload.service_token())
        .await?;

    device_service
        .unbind_device_from_user(user_data.id, &payload.device_id)
        .await?;
    let response: BaseResult<()> = BaseResult {
        code: 0,
        data: None,
        info: Some(String::from("Unbind success")),
    };

    Ok(Json(response))
}

pub async fn logout(
    State(user_service): State<UserService>,
    Form(payload): Form<LogoutRequest>,
) -> Result<Json<BaseResult<()>>, AppError> {
    log::info!("New logout request: {}", payload);

    payload.validate_auth()?;
    user_service
        .get_user_data_by_service_token(payload.service_token())
        .await?;

    let response: BaseResult<()> = BaseResult {
        code: 0,
        data: None,
        info: Some(String::from("Logout success")),
    };

    Ok(Json(response))
}

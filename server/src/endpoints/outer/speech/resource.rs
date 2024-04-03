use axum::body::{Body, Bytes};
use axum::extract::{Path, Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use AppError::OpenAIError;

use crate::endpoints::common::DATE_FORMAT;
use crate::endpoints::errors::AppError;
use crate::endpoints::language::Language;
use crate::endpoints::models::{BaseResult, UrlResponse};

use crate::common::services::file::FileService;
use crate::endpoints::outer::speech::models::{QaRequest, TranslateRequest};
use crate::{opus, AppState};

pub async fn qa(
    State(AppState {
        device_service,
        openai,
        history_service,
        file_service,
        ..
    }): State<AppState>,
    Query(payload): Query<QaRequest>,
    mut body: Bytes,
) -> Result<Json<BaseResult<UrlResponse>>, AppError> {
    log::info!("New sound network request: {}", payload);
    log::info!("Bytes uploaded: {}", body.len());

    let device_data = device_service
        .get_device_data_by_device_id(&payload.device_id)
        .await?;

    let timestamp = chrono::Utc::now().format(DATE_FORMAT);
    let request_filename = format!("qa-{}.ogg", timestamp);
    let response_filename = format!("qa-{}-response.mp3", timestamp);

    let packets = opus::container::create_container(&mut body);
    file_service.save_audio_file(&request_filename, &packets)?;

    let from = &device_data.language_from;

    let conversation = openai
        .qa(packets, from)
        .await
        .map_err(|e| OpenAIError(e.to_string()))?;
    history_service
        .save_chat_record(
            from,
            from,
            &conversation.request_text,
            &conversation.response_text,
            device_data.id,
            true,
        )
        .await?;

    file_service
        .save_file_and_return_url(&response_filename, &conversation.response)
        .map(|url| Json(BaseResult::new(UrlResponse::new(url))))
}

pub async fn translate(
    State(AppState {
        device_service,
        openai,
        history_service,
        file_service,
        ..
    }): State<AppState>,
    Query(payload): Query<TranslateRequest>,
    mut body: Bytes,
) -> Result<Json<BaseResult<UrlResponse>>, AppError> {
    log::info!("New sound network request: {}", payload);
    log::info!("Bytes uploaded: {}", body.len());

    let device_data = device_service
        .get_device_data_by_device_id(&payload.device_id)
        .await?;

    let from = Language::from_device_code(&payload.from)?.iso_code;
    let to = Language::from_device_code(&payload.to)?.iso_code;
    let timestamp = chrono::Utc::now().format(DATE_FORMAT);
    let request_filename = format!("translate-{}-{}-{}.ogg", timestamp, from, to);
    let response_filename = format!("translate-{}-{}-{}-response.mp3", timestamp, from, to);

    let packets = opus::container::create_container(&mut body);
    file_service.save_audio_file(&request_filename, &packets)?;

    let conversation = openai
        .translate(packets, &from, &to)
        .await
        .map_err(|err| OpenAIError(err.to_string()))?;

    let ismyself = device_data.language_to.eq(&to);

    history_service
        .save_chat_record(
            &from,
            &to,
            &conversation.request_text,
            &conversation.response_text,
            device_data.id,
            ismyself,
        )
        .await?;

    file_service
        .save_file_and_return_url(&response_filename, &conversation.response)
        .map(|url| Json(BaseResult::new(UrlResponse::new(url))))
}

pub async fn download_audio(
    State(file_service): State<FileService>,
    Path(file_name): Path<String>,
) -> Result<Response, Response> {
    let file = file_service
        .get_file_stream(&file_name)
        .await
        .map_err(|_| StatusCode::NOT_FOUND.into_response())?;

    Ok((
        [(header::CONTENT_TYPE, "audio/mpeg")],
        Body::from_stream(file),
    )
        .into_response())
}

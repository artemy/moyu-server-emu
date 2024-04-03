use axum::extract::{FromRequest, Request};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::{Json};
use serde::Deserialize;

pub struct PlainJson<T>(pub T);

impl<S, T> FromRequest<S> for PlainJson<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    T: for<'de> Deserialize<'de> + Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json")
                || content_type.starts_with("text/plain")
            {
                let body = hyper::body::Bytes::from_request(req, &())
                    .await
                    .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
                return match serde_json::from_slice::<T>(&body) {
                    Ok(val) => Ok(PlainJson(val)),
                    Err(_) => Err((StatusCode::BAD_REQUEST, String::from("Invalid JSON"))),
                };
            }
        }

        Err((
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            String::from("Unsupported media type"),
        ))
    }
}

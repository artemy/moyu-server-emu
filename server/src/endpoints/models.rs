use serde::Serialize;

#[derive(Serialize)]
pub struct BaseResult<T> {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}

impl<T> BaseResult<T> {
    pub fn new(input: T) -> Self {
        BaseResult {
            code: 0,
            data: Some(input),
            info: None,
        }
    }
    pub fn none() -> BaseResult<()> {
        BaseResult {
            code: 0,
            data: None,
            info: None,
        }
    }
}

#[derive(Serialize)]
pub struct BaseList<T> {
    pub count: u32,
    pub list: Vec<T>,
    pub more: bool,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub url: String,
}

impl UrlResponse {
    pub fn new(url: String) -> Self {
        UrlResponse { url }
    }
}

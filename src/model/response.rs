use super::error::AppError;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

mod response_message;

#[derive(Debug, Clone)]
pub struct GeneralResponse {
    pub status: StatusCode,
    pub body: String,
}

// NOTE: General response for all layer and handler
impl GeneralResponse {
    pub fn new<T: Serialize>(status: StatusCode, data: T) -> Result<GeneralResponse, AppError> {
        let message = get_general_message(&status);
        let body_obj = GeneralBody::new(status, message, Some(data));
        let body = serde_json::to_string(&body_obj)?;

        let res = GeneralResponse { status, body };
        Ok(res)
    }

    pub fn new_general(
        status: StatusCode,
        message: Option<String>,
    ) -> Result<GeneralResponse, AppError> {
        let message = if let Some(msg) = message {
            msg
        } else {
            get_general_message(&status)
        };

        let general_body = GeneralBody::<bool>::new(status, message, None);
        let body = serde_json::to_string(&general_body)?;

        let res = GeneralResponse { status, body };
        Ok(res)
    }

    pub fn ok_with_data<T: Serialize>(result: T) -> Result<GeneralResponse, AppError> {
        let status = StatusCode::OK;
        let general_body = GeneralBody::new(status, get_general_message(&status), Some(result));
        let body = serde_json::to_string(&general_body)?;

        let res = GeneralResponse { status, body };
        Ok(res)
    }
}

impl IntoResponse for GeneralResponse {
    fn into_response(self) -> axum::response::Response {
        let mut header = HeaderMap::new();
        header.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        (self.status, header, self.body).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralBody<T> {
    data: Option<T>,
    status: u16,
    message: String,
}

impl<T: Serialize> GeneralBody<T> {
    pub fn new(status: StatusCode, message: String, data: Option<T>) -> GeneralBody<T> {
        let status = status.as_u16();
        GeneralBody {
            data,
            status,
            message,
        }
    }
}

fn get_general_message(status: &StatusCode) -> String {
    match status {
        &StatusCode::OK => response_message::OK,
        &StatusCode::UNAUTHORIZED => response_message::UNAUTHORIZED,
        &StatusCode::INTERNAL_SERVER_ERROR => response_message::INTERNAL_SERVER_ERROR,
        &StatusCode::BAD_REQUEST => response_message::BAD_REQUEST,
        _ => response_message::UNDEFINED,
    }
    .to_string()
}

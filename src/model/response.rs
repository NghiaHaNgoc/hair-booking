use super::error::AppError;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

mod response_message;

#[derive(Debug, Clone)]
pub struct GeneralResponse {
    pub header: HeaderMap,
    pub body: String,
}

// NOTE: General response for all layer and handler
impl GeneralResponse {
    pub fn new<T: Serialize>(
        header: HeaderMap,
        message_code: &str,
        data: T,
    ) -> Result<GeneralResponse, AppError> {
        let body_obj = GeneralBody::new(message_code.to_string(), Some(data));
        let body = serde_json::to_string(&body_obj)?;

        let res = GeneralResponse { header, body };
        Ok(res)
    }

    pub fn new_general(message_code: &str) -> Result<GeneralResponse, AppError> {
        let general_body = GeneralBody::<bool>::new(message_code.to_string(), None);
        let body = serde_json::to_string(&general_body)?;

        let res = GeneralResponse {
            header: HeaderMap::new(),
            body,
        };
        Ok(res)
    }

    pub fn new_error(message: String) -> Result<Self, AppError> {
        let general_body = GeneralBody::<bool>::new_custom("G0004".to_string(), message, None);
        let body = serde_json::to_string(&general_body)?;
        let res = GeneralResponse {
            header: HeaderMap::new(),
            body,
        };
        Ok(res)
    }

    pub fn ok_with_data<T: Serialize>(data: T) -> Result<GeneralResponse, AppError> {
        let message_code = "G0001".to_string();
        let general_body = GeneralBody::new(message_code, Some(data));
        let body = serde_json::to_string(&general_body)?;

        let res = GeneralResponse {
            header: HeaderMap::new(),
            body,
        };
        Ok(res)
    }
}

impl IntoResponse for GeneralResponse {
    fn into_response(mut self) -> axum::response::Response {
        self.header.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        (StatusCode::OK, self.header, self.body).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GeneralBody<T> {
    data: Option<T>,
    message_code: String,
    message: String,
}

impl<T: Serialize> GeneralBody<T> {
    pub fn new(message_code: String, data: Option<T>) -> GeneralBody<T> {
        let message = match response_message::response_message().get(message_code.as_str()) {
            Some(msg) => msg.to_string(),
            None => "Undefied!".to_string(),
        };
        GeneralBody {
            data,
            message_code,
            message,
        }
    }

    pub fn new_custom(message_code: String, message: String, data: Option<T>) -> GeneralBody<T> {
        GeneralBody {
            data,
            message_code,
            message,
        }
    }
}

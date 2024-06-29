use super::response::GeneralResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl AppError {
    pub fn new(err_message: String) -> Self {
        let err = anyhow::anyhow!(err_message);
        AppError(err)
    }
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = format!("Error: {}", self.0);
        let res = GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message));
        res.into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

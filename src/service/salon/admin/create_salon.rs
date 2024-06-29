use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    database::Salon,
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateSalonInput {
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    post,
    tag = "Salon",
    path = "/admin/salon",
    responses(
        (status = 200, description = "Create salon by admin")
    )
)]
pub async fn create_salon(
    State(db): State<Arc<Postgrest>>,
    Json(create_salon_input): Json<CreateSalonInput>,
) -> Result<GeneralResponse, AppError> {
    //Hash password
    let signup_input_json = serde_json::to_string(&create_salon_input)?;

    let query = db
        .from("salons")
        .insert(signup_input_json)
        .single()
        .execute()
        .await
        .unwrap();

    if query.status().is_success() {
        let data: Salon = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

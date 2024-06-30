use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
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
    delete,
    tag = "Salon",
    path = "/admin/salon/{salon_id}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete salon by admin")
    )
)]
pub async fn delete_salon(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salons")
        .eq("id", salon_id.to_string())
        .single()
        .delete()
        .execute()
        .await?;

    if query.status().is_success() {
        let data: Salon = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = "salon_id not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    claim::Claims, database::SalonOuput, error::AppError, response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct UpdateSalonInput {
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
}

#[utoipa::path(
    put,
    tag = "Salon",
    path = "/salon-user/salon/{salonId}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Update salon by salon user")
    )
)]
pub async fn update_salon(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
    claims: Claims,
    Json(update_salon_input): Json<UpdateSalonInput>,
) -> Result<GeneralResponse, AppError> {
    let input_json = serde_json::to_string(&update_salon_input)?;
    let query_salon = db
        .from("salons")
        .eq("id", salon_id.to_string())
        .eq("user_id", claims.id.to_string())
        .update(input_json)
        .single()
        .execute()
        .await?;

    if query_salon.status().is_success() {
        let salon: SalonOuput = query_salon.json().await?;
        GeneralResponse::ok_with_data(salon)
    } else {
        let message = "Nothing changed.".to_string();
        GeneralResponse::new_general(StatusCode::NOT_MODIFIED, Some(message))
    }
}

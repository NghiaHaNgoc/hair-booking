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
    claim::Claims,
    database::{MediaType, SalonMediaOutput, SalonOuput},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateSalonMediaInput {
    pub url: String,
    pub media_type: MediaType,
    #[serde(skip_deserializing)]
    pub salon_id: Option<u64>,
}

#[utoipa::path(
    post,
    tag = "Salon",
    path = "/salon-user/salon/{salonId}/media",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Add media (image, video) for salon by salon user")
    )
)]
pub async fn create_salon_media(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
    claims: Claims,
    Json(mut input): Json<CreateSalonMediaInput>,
) -> Result<GeneralResponse, AppError> {
    // Verify owner salon
    let query_salon_owner = db
        .from("salons")
        .select("id")
        .eq("id", salon_id.to_string())
        .eq("user_id", claims.id.to_string())
        .single()
        .execute()
        .await?;
    if !query_salon_owner.status().is_success() {
        let message = "salon not found or not your salon.".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    input.salon_id = Some(salon_id);
    let input_json = serde_json::to_string(&input)?;
    let query = db
        .from("salon_medias")
        .insert(input_json)
        .single()
        .execute()
        .await?;

    if query.status().is_success() {
        let salon: SalonMediaOutput = query.json().await?;
        GeneralResponse::ok_with_data(salon)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
    }
}

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::{
    claim::Claims, database::SalonBedOutput, error::AppError, response::GeneralResponse,
};

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateSalonBedInput {
    name: String,
    #[serde(skip_deserializing)]
    salon_id: Option<u64>,
}

#[utoipa::path(
    post,
    tag = "Salon bed",
    path = "/salon-user/salon/{salonId}/salon-bed",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Create salon bed by salon user")
    )
)]
pub async fn create_salon_bed(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
    claims: Claims,
    Json(mut input): Json<CreateSalonBedInput>,
) -> Result<GeneralResponse, AppError> {
    let verify_salon_owner_query = db
        .from("salons")
        .select("id")
        .eq("id", salon_id.to_string())
        .eq("user_id", claims.id.to_string())
        .single()
        .execute()
        .await?;
    if !verify_salon_owner_query.status().is_success() {
        let message = "salon not found of not your salon.".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }

    input.salon_id = Some(salon_id);
    let input_json = serde_json::to_string(&input)?;

    let query_salon = db
        .from("salon_beds")
        .insert(input_json)
        .single()
        .execute()
        .await?;

    if query_salon.status().is_success() {
        let salon_bed: SalonBedOutput = query_salon.json().await?;

        GeneralResponse::ok_with_data(salon_bed)
    } else {
        let message = query_salon.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

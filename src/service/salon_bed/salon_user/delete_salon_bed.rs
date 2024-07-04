use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{
    claim::Claims, database::SalonBedOutput, error::AppError, response::GeneralResponse,
};

#[utoipa::path(
    delete,
    tag = "Salon bed",
    path = "/salon-user/salon-bed/{salonBedId}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete salon bed by salon user")
    )
)]
pub async fn delete_salon_bed(
    State(db): State<Arc<Postgrest>>,
    Path(salon_bed_id): Path<u64>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salon_beds")
        .select("*, salons(id, user_id)")
        .eq("id", salon_bed_id.to_string())
        .eq("salons.user_id", claims.id.to_string())
        .delete()
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let data: SalonBedOutput = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = "salon bed not found or not your salon bed.".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

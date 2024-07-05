use std::sync::Arc;

use axum::extract::{Path, State};
use postgrest::Postgrest;

use crate::model::{database::SalonBedOutput, error::AppError, response::GeneralResponse};

#[utoipa::path(
    get,
    tag = "Salon bed",
    path = "/public/salon/{salonId}/salon-bed",
    responses(
        (status = 200, description = "Get list of salon")
    )
)]

pub async fn list_salon_bed(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salon_beds")
        .select("*")
        .eq("salon_id", salon_id.to_string())
        .order("id.asc")
        .execute()
        .await?;

    let data: Vec<SalonBedOutput> = query.json().await?;
    GeneralResponse::ok_with_data(data)
}

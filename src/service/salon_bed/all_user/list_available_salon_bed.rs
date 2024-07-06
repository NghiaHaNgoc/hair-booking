use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::IntoParams;

use crate::model::{database::SalonBedOutput, error::AppError, response::GeneralResponse};

#[derive(IntoParams, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[into_params(rename_all = "camelCase")]
pub struct TimeRangeQuery {
    time_from: DateTime<Utc>,
    time_to: DateTime<Utc>,
}

#[utoipa::path(
    get,
    tag = "Salon bed",
    path = "/all-user/salon/{salonId}/available-salon-bed",
    security(("Authorization" = [])),
    params(TimeRangeQuery),
    responses(
        (status = 200, description = "Get list of available salon bed")
    )
)]

pub async fn list_available_salon_bed(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
    Query(time_range): Query<TimeRangeQuery>,
) -> Result<GeneralResponse, AppError> {
    if time_range.time_from >= time_range.time_to {
        let message = "time to must be greater than time from".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    // let query_json = serde_json::to_string(&time_range)?;
    let query_json = json!({
        "p_time_from": time_range.time_from,
        "p_time_to": time_range.time_to
    })
    .to_string();
    let query = db
        .rpc("get_available_salon_beds", query_json)
        .eq("salon_id", salon_id.to_string())
        .execute()
        .await?;

    let data: Vec<SalonBedOutput> = query.json().await?;
    GeneralResponse::ok_with_data(data)
}

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::IntoParams;

use crate::model::{database::SalonBed, error::AppError, response::GeneralResponse};



const LIST_SALON_BED_QUERY: &str = "
SELECT * FROM salon_beds
WHERE branch_id = $1
";

/// List bed of a branch
#[utoipa::path(
    get,
    tag = "Salon bed",
    path = "/public/branch/{branchId}/salon-bed",
)]
pub async fn list_bed(
    State(db): State<Arc<Pool<Postgres>>>,
    Path(branch_id): Path<i64>
) -> Result<GeneralResponse, AppError> {
    let beds: Vec<SalonBed> = sqlx::query_as(LIST_SALON_BED_QUERY)
        .bind(branch_id)
        .fetch_all(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(beds)
}

// ---------------------------------------------------------------

#[derive(IntoParams, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[into_params(rename_all = "camelCase")]
pub struct TimeRangeQuery {
    #[param(value_type = String, example = "2024-01-01T00:00:00Z")]
    time_from: DateTime<Utc>,
    #[param(value_type = String, example = "2024-01-01T00:00:00Z")]
    time_to: DateTime<Utc>,
}

const AVAILABLE_BED_QUERY: &str = "
SELECT *
FROM salon_beds
WHERE salon_beds.branch_id = $1
AND salon_beds.id NOT IN (
  SELECT reservations.salon_bed_id
  FROM reservations
  WHERE 
  reservations.status = 'WAITING'
  AND ((reservations.time_from, reservations.time_to)
  OVERLAPS ($2::timestamptz, $3::timestamptz))
)
";

/// List available bed of a branch
#[utoipa::path(
    get,
    tag = "Salon bed",
    params(TimeRangeQuery),
    path = "/public/branch/{branchId}/available-bed",
)]
pub async fn available_bed(
    State(db): State<Arc<Pool<Postgres>>>,
    Path(branch_id): Path<i64>,
    Query(time_range): Query<TimeRangeQuery>
) -> Result<GeneralResponse, AppError> {
    if time_range.time_from >= time_range.time_to {
        return GeneralResponse::new_error("timeTo must be greater than timeFrom!".to_string());
    }
    let beds: Vec<SalonBed> = sqlx::query_as(AVAILABLE_BED_QUERY)
        .bind(branch_id)
        .bind(time_range.time_from)
        .bind(time_range.time_to)
        .fetch_all(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(beds)
}

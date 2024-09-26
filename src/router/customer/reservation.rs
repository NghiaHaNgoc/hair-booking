use std::sync::Arc;

use anyhow::anyhow;
use axum::{extract::{Query, State}, http::StatusCode, Extension, Json};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use sqlx::{FromRow, Pool, Postgres, Row};
use utoipa::ToSchema;

use crate::{model::{claim::Claims, database::{GeneralPagingQueryInput, Reservation, ReservationOutput}, error::AppError, response::GeneralResponse}, utils};





#[derive(ToSchema, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct AddReservationInput {
    pub therapy_id: i64,
    pub salon_bed_id: i64,
    pub time_from: DateTime<Utc>,
    pub time_to: DateTime<Utc>,
    pub comment: Option<String>,
}

const ADD_RESERVATION_QUERY: &str = "
INSERT INTO reservations
(user_id, therapy_id, salon_bed_id, time_from, time_to, comment)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING *
";

/// Add reservation of customer
#[utoipa::path(
    post,
    tag = "Reservation",
    path = "/customer/reservation",
    security(("Authorization" = [])),
)]
pub async fn add_reservation(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Json(input): Json<AddReservationInput>,
) -> Result<GeneralResponse, AppError> {
    if input.time_from <= Utc::now() {
        return GeneralResponse::new_error("timeFrom must be greater than time now!".to_string());
    }
    if input.time_from >= input.time_to {
        return GeneralResponse::new_error("timeTo must be greater than timeFrom!".to_string());
    }
    
    let _validate_field = sqlx::query("
SELECT salon_branches.* FROM salon_branches
INNER JOIN therapies ON therapies.salon_id = salon_branches.salon_id
INNER JOIN salon_beds ON salon_beds.branch_id = salon_branches.id
WHERE therapies.id = $1
AND salon_beds.id = $2
")
        .bind(input.therapy_id)
        .bind(input.salon_bed_id)
        .fetch_one(db.as_ref())
        .await
    .map_err(|_| anyhow!("therapy and salon bed are not in same salon!"))?;

    let _validate_available = sqlx::query("
SELECT *
FROM salon_beds
WHERE salon_beds.id = $1
AND salon_beds.id NOT IN (
  SELECT reservations.salon_bed_id
  FROM reservations
  WHERE 
  reservations.status = 'WAITING'
  AND ((reservations.time_from, reservations.time_to)
  OVERLAPS ($2::timestamptz, $3::timestamptz))
)
")
        .bind(input.salon_bed_id)
        .bind(input.time_from)
        .bind(input.time_to)
        .fetch_one(db.as_ref())
        .await
    .map_err(|_| anyhow!("This bed not found or booked at this time!"))?;

    let _reservation: Reservation = sqlx::query_as(ADD_RESERVATION_QUERY)
        .bind(claims.id)
        .bind(input.therapy_id)
        .bind(input.salon_bed_id)
        .bind(input.time_from)
        .bind(input.time_to)
        .bind(input.comment)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::new_general(StatusCode::OK)
}

// -----------------------------------------------------------------------------

const LIST_RESERVATION_QUERY: &str = "
SELECT reservations.*,
to_jsonb(salon_beds) as salon_bed,
to_jsonb(salon_branches) as salon_branch,
to_jsonb(salons) as salon,
to_jsonb(therapies) as therapy,
COUNT(*) OVER () AS total
FROM reservations
LEFT JOIN therapies ON therapies.id = reservations.therapy_id
LEFT JOIN salon_beds ON salon_beds.id = reservations.salon_bed_id
LEFT JOIN salon_branches ON salon_branches.id = salon_beds.branch_id
LEFT JOIN salons ON salons.id = salon_branches.salon_id
WHERE reservations.user_id = $1
ORDER BY reservations.time_from DESC
OFFSET $2
LIMIT $3
";

/// Get list of reservation of this customer
#[utoipa::path(
    get,
    tag = "Reservation",
    path = "/customer/reservation",
    security(("Authorization" = [])),
    params(GeneralPagingQueryInput)
)]
pub async fn list_reservation(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Query(GeneralPagingQueryInput { offset, limit }): Query<GeneralPagingQueryInput>,
) -> Result<GeneralResponse, AppError> {

    let salons = sqlx::query(LIST_RESERVATION_QUERY)
        .bind(claims.id)
        .bind(offset)
        .bind(limit)
        .fetch_all(db.as_ref())
        .await?;

    let mut total: Option<i64> = None;
    let reservations: Vec<ReservationOutput> = salons
        .into_iter()
        .map(|reservation| {
            if total == None {
                total = reservation.try_get("total").ok();
            }
            let reservation = ReservationOutput::from_row(&reservation).unwrap_or_default();
            reservation
        })
        .collect();

    let total = total.unwrap_or(0);

    let data = json!({
        "reservations": reservations,
        "total": total
    });
    GeneralResponse::ok_with_data(data)
}

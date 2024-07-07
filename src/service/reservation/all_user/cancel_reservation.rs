use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    claim::Claims, database::ReservationOuput, error::AppError, response::GeneralResponse,
};

#[utoipa::path(
    put,
    tag = "Reservation",
    path = "/all-user/reservation/{reservationId}/cancel",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Cancel a reservation")
    )
)]
pub async fn cancel_reservation(
    State(db): State<Arc<Postgrest>>,
    Path(reservation_id): Path<u64>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("reservations")
        .eq("id", reservation_id.to_string())
        .eq("user_id", claims.id.to_string())
        .update(r#"{"status": "CANCEL"}"#)
        .select("*, salon_bed:salon_beds(salon_id)")
        .single()
        .execute()
        .await?;

    if query.status().is_success() {
        let mut reservation: ReservationOuput = query.json().await?;
        if let Some(salon_bed) = reservation.salon_bed.as_ref() {
            reservation.salon_id = salon_bed.salon_id;
        };

        GeneralResponse::ok_with_data(reservation)
    } else {
        let message = "This reservation of this user already canceled or not found.".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

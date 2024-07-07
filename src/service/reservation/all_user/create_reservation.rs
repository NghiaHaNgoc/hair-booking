
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    claim::Claims, database::ReservationOuput, error::AppError, response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateReservationInput {
    pub salon_bed_id: u64,
    pub time_from: DateTime<Utc>,
    pub time_to: DateTime<Utc>,
    pub comment: Option<String>,
    #[serde(skip_deserializing)]
    pub user_id: Option<u64>
}
#[utoipa::path(
    post,
    tag = "Reservation",
    path = "/all-user/reservation",
    security(("Authorization" = [])),
    request_body = CreateReservationInput,
    responses(
        (status = 200, description = "Create a reservation")
    )
)]
pub async fn create_reservation(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
    Json(mut input): Json<CreateReservationInput>,
) -> Result<GeneralResponse, AppError> {
    if input.time_from >= input.time_to {
        let message = "time to must be greater than time from".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    let query_json = json!({
        "p_time_from": input.time_from,
        "p_time_to": input.time_to
    })
    .to_string();
    let available_query = db
        .rpc("get_available_salon_beds", query_json)
        .eq("id", input.salon_bed_id.to_string())
        .single()
        .execute()
        .await?;

    if !available_query.status().is_success() {
        let message = "This bed already booked at this time.".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    input.user_id = Some(claims.id);

    let input_json = serde_json::to_string(&input)?;

    let query = db
        .from("reservations")
        .insert(input_json)
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
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

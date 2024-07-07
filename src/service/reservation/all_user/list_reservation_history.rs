use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::{
    model::{
        claim::Claims,
        database::{GeneralPagingQueryInput, ReservationOuput},
        error::AppError,
        response::GeneralResponse,
    },
    utils,
};

#[utoipa::path(
    get,
    tag = "Reservation",
    path = "/all-user/reservation",
    security(("Authorization" = [])),
    params(
        GeneralPagingQueryInput
    ),
    responses(
        (status = 200, description = "Cancel a reservation")
    )
)]
pub async fn list_reservation_history(
    State(db): State<Arc<Postgrest>>,
    Query(query_params): Query<GeneralPagingQueryInput>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let (page, limit) = utils::extract_page_and_limit(query_params.page, query_params.limit);
    let (from_index, to_index) = utils::get_query_from_to(page, limit)?;

    let query = db
        .from("reservations")
        .select("*")
        .eq("user_id", claims.id.to_string())
        .order("time_from.desc")
        .exact_count()
        .range(from_index, to_index).execute().await?;

    let total = utils::total_from_header(query.headers())?;
    let pages = utils::total_pages(total, limit);
    if query.status().is_success() {
        let salons: Vec<ReservationOuput> = query.json().await?;
        let data = json!({
            "users": salons,
            "pages": pages,
            "total": total
        });
        GeneralResponse::ok_with_data(data)
    } else {
        let salons: Vec<ReservationOuput> = Vec::new();
        let data = json!({
            "users": salons,
            "pages": pages,
            "total": total
        });
        GeneralResponse::ok_with_data(data)
    }
}

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{
    claim::Claims, database::SalonOuput, error::AppError, response::GeneralResponse,
};

#[utoipa::path(
    delete,
    tag = "Salon",
    path = "/salon-user/salon/{salonId}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete salon by salon user")
    )
)]
pub async fn delete_salon(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salons")
        .eq("id", salon_id.to_string())
        .eq("user_id", claims.id.to_string())
        .single()
        .delete()
        .execute()
        .await?;

    if query.status().is_success() {
        let data: SalonOuput = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = "salon_id for this user not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

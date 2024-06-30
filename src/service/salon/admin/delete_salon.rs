use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{database::Salon, error::AppError, response::GeneralResponse};

#[utoipa::path(
    delete,
    tag = "Salon",
    path = "/admin/salon/{salon_id}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete salon by admin")
    )
)]
pub async fn delete_salon(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salons")
        .eq("id", salon_id.to_string())
        .single()
        .delete()
        .execute()
        .await?;

    if query.status().is_success() {
        let data: Salon = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = "salon_id not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

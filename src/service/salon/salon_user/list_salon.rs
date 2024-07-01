use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;

use crate::model::{claim::Claims, database::SalonOuput, error::AppError, response::GeneralResponse};

#[utoipa::path(
    get,
    tag = "Salon",
    path = "/salon-user/salon",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "List salon by salon user")
    )
)]
pub async fn list_salon(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salons")
        .select("*")
        .eq("user_id", claims.id.to_string())
        .execute()
        .await?;

    if query.status().is_success() {
        let data: Vec<SalonOuput> = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}

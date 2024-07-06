use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{
    claim::Claims, database::SalonMediaOutput, error::AppError, response::GeneralResponse,
};

#[utoipa::path(
    delete,
    tag = "Salon",
    path = "/salon-user/salon/{salonId}/media/{mediaId}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete salon media by salon user")
    )
)]
pub async fn delete_salon_media(
    State(db): State<Arc<Postgrest>>,
    Path((salon_id, media_id)): Path<(u64, u64)>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salon_medias")
        .select("*, salon:salon_id(user_id)")
        .eq("id", media_id.to_string())
        .eq("salon_id", salon_id.to_string())
        .eq("salon.user_id", claims.id.to_string())
        .single()
        .delete()
        .execute()
        .await?;

    if query.status().is_success() {
        let data: SalonMediaOutput = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        println!("{}", query.text().await?);
        let message = "media for this salon not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

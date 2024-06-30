use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use postgrest::Postgrest;

use crate::model::{
    database::{User, UserRole},
    error::AppError,
    response::GeneralResponse,
};

#[utoipa::path(
    delete,
    tag = "User",
    path = "/admin/user/{user_id}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete user have role CUSTOMER or SALON_USER by admin")
    )
)]
pub async fn delete_user(
    State(db): State<Arc<Postgrest>>,
    Path(user_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("users")
        .eq("id", user_id.to_string())
        .or(format!(
            "role.eq.{}, role.eq.{}",
            UserRole::SalonUser,
            UserRole::Customer
        ))
        .single()
        .delete()
        .execute()
        .await?;

    if query.status().is_success() {
        let data: User = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = "user_id is invalid!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

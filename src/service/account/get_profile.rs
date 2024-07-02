use crate::model::{
    claim::Claims, database::UserOutput, error::AppError, response::GeneralResponse,
};
use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;
use std::sync::Arc;

const QUERY_FIELD: [&str; 8] = [
    "id",
    "username",
    "full_name",
    "email",
    "gender",
    "role",
    "avatar",
    "created_at",
];

#[utoipa::path(
    get,
    tag = "Account",
    path = "/account/profile",
    security(("Authorization" = []))
)]
pub async fn get_profile(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    let query_field = QUERY_FIELD.join(", ");
    let query = db
        .from("users")
        .select(query_field)
        .eq("id", claims.id.to_string())
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let profile: UserOutput = query.json().await?;
        GeneralResponse::ok_with_data(profile)
    } else {
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}

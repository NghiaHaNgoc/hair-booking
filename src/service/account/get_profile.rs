use crate::model::{
    claim::Claims,
    database::{Gender, UserRole},
    error::AppError,
    response::GeneralResponse,
};
use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct UserProfileOutput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<Gender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub created_at: Option<String>,
    pub salon_id: Option<u64>,
}

const QUERY_FIELD: [&str; 9] = [
    "username",
    "email",
    "address",
    "date_of_birth",
    "gender",
    "role",
    "avatar",
    "created_at",
    "salon_id",
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
        let profile: UserProfileOutput = query.json().await?;
        GeneralResponse::ok_with_data(profile)
    } else {
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}

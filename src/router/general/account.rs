use crate::model::{
    claim::Claims,
    database::{UserGender, UserOutput},
    error::AppError,
    response::GeneralResponse,
};
use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use utoipa::ToSchema;

const GET_PROFILE_QUERY: &str = "SELECT * FROM users WHERE id = $1";

/// Get profile
#[utoipa::path(
    get,
    tag = "Account",
    path = "/account/profile",
    security(("Authorization" = []))
)]
pub async fn get_profile(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
) -> Result<GeneralResponse, AppError> {
    let user: UserOutput = sqlx::query_as(GET_PROFILE_QUERY)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await?;
    GeneralResponse::ok_with_data(user)
}

// ---------------------------------------------------------------

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct UpdateUserProfileInput {
    pub full_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub avatar: Option<String>,
}

const UPDATE_PROFILE_QUERY: &str = "UPDATE users set
full_name = COALESCE($1, full_name),
date_of_birth = COALESCE($2, date_of_birth),
email = COALESCE($3, email),
gender = COALESCE($4, gender),
avatar = COALESCE($5, avatar)
where id = $6
";

/// Update profile
#[utoipa::path(
    put,
    tag = "Account",
    path = "/account/profile",
    security(("Authorization" = []))
)]
pub async fn update_profile(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Json(input): Json<UpdateUserProfileInput>,
) -> Result<GeneralResponse, AppError> {
    let user: UserOutput = sqlx::query_as(UPDATE_PROFILE_QUERY)
        .bind(input.full_name)
        .bind(input.date_of_birth)
        .bind(input.email)
        .bind(input.gender)
        .bind(input.avatar)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await?;
    GeneralResponse::ok_with_data(user)
}

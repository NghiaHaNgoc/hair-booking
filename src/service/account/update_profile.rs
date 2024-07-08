use crate::model::{
    claim::Claims, database::{UserGender, UserOutput, UserRole}, error::AppError, response::GeneralResponse,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
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

#[skip_serializing_none]
#[derive(ToSchema,Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct UpdateUserProfileInput {
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub date_of_birth: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub avatar: Option<String>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[utoipa::path(
    put,
    tag = "Account",
    path = "/account/profile",
    security(("Authorization" = []))
)]
pub async fn update_profile(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
    Json(mut input): Json<UpdateUserProfileInput>
) -> Result<GeneralResponse, AppError> {
    input.updated_at = Some(Utc::now());
    let input_json = serde_json::to_string(&input)?;
    let query = db
        .from("users")
        .eq("id", claims.id.to_string())
        .update(input_json)
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

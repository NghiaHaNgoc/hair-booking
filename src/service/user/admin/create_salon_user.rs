use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    database::{UserOutput, UserGender, UserRole},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateSalonUserInput {
    username: String,
    password: String,
    email: Option<String>,
    gender: Option<UserGender>,
    #[serde(skip_deserializing)]
    role: Option<UserRole>,
}

#[utoipa::path(
    post,
    tag = "User",
    path = "/admin/user/salon-user",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Create salon user by admin")
    )
)]
pub async fn create_salon_user(
    State(db): State<Arc<Postgrest>>,
    Json(mut create_user_input): Json<CreateSalonUserInput>,
) -> Result<GeneralResponse, AppError> {
    //Hash password
    let password_hash = bcrypt::hash(create_user_input.password, 4)?;
    create_user_input.password = password_hash;

    // Add role
    create_user_input.role = Some(UserRole::SalonUser);

    let user_input_json = serde_json::to_string(&create_user_input)?;

    let query = db
        .from("users")
        .insert(user_input_json)
        .single()
        .execute()
        .await
        .unwrap();

    if query.status().is_success() {
        let user: UserOutput = query.json().await?;
        GeneralResponse::ok_with_data(user)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

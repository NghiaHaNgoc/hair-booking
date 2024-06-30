use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    claim::Claims,
    database::{GeneralUserOutput, User, UserGender, UserRole},
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
    address: Option<String>,
    date_of_birth: Option<String>,
    gender: Option<UserGender>,
    #[serde(skip_deserializing)]
    role: Option<UserRole>,
    #[serde(skip_deserializing)]
    salon_id: Option<u64>,
}

#[utoipa::path(
    post,
    tag = "User",
    path = "/admin/user/salon/{salon_id}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Create user salon by admin")
    )
)]
pub async fn create_salon_user(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
    Json(mut create_user_input): Json<CreateSalonUserInput>,
) -> Result<GeneralResponse, AppError> {
    create_user_input.salon_id = Some(salon_id);
    //Hash password
    let password_hash = bcrypt::hash(create_user_input.password, 4)?;
    create_user_input.password = password_hash;

    // Add role
    create_user_input.role = Some(UserRole::SalonUser);

    let signup_input_json = serde_json::to_string(&create_user_input)?;

    let query = db
        .from("users")
        .insert(signup_input_json)
        .single()
        .execute()
        .await
        .unwrap();

    if query.status().is_success() {
        let user: GeneralUserOutput = query.json().await?;
        GeneralResponse::ok_with_data(user)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

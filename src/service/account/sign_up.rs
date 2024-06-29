use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    claim::Claims,
    database::{User, UserGender, UserRole},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct SignupInput {
    username: String,
    password: String,
    email: Option<String>,
    address: Option<String>,
    date_of_birth: Option<String>,
    gender: Option<UserGender>,
    #[serde(skip_deserializing)]
    role: Option<UserRole>,
}

#[utoipa::path(post, tag = "Account", path = "/account/sign-up")]
pub async fn sign_up(
    State(db): State<Arc<Postgrest>>,
    Json(mut signup_input): Json<SignupInput>,
) -> Result<GeneralResponse, AppError> {
    //Hash password
    let password_hash = bcrypt::hash(signup_input.password, 4)?;
    signup_input.password = password_hash;

    // Add role
    signup_input.role = Some(UserRole::CUSTOMER);

    let signup_input_json = serde_json::to_string(&signup_input)?;

    let query = db
        .from("users")
        .insert(signup_input_json)
        .single()
        .execute()
        .await
        .unwrap();

    if query.status().is_success() {
        let user: User = query.json().await?;
        let token = Claims::create_token(&user)?;
        let data = json!({
            "username": user.username,
            "email": user.email,
            "role": user.role,
            "avatar": user.avatar,
            "token": token
        });
        GeneralResponse::ok_with_data(data)
    } else {
        let message = query.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

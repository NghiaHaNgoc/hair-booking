use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::model::{claim::Claims, database::User, error::AppError, response::GeneralResponse};

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct LoginInput {
    username: String,
    password: String,
}

const QUERY_FIELD: [&str; 6] = ["id", "username", "email", "password", "role", "avatar"];

#[utoipa::path(post, tag = "Account", path = "/account/sign-in")]

pub async fn sign_in(
    State(db): State<Arc<Postgrest>>,
    Json(login_input): Json<LoginInput>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("users")
        .select(QUERY_FIELD.join(", "))
        .eq("username", login_input.username)
        .single()
        .execute()
        .await
        .unwrap();

    if query.status().is_success() {
        let user: User = query.json().await?;
        let result_verify = bcrypt::verify(login_input.password, user.password.as_ref().unwrap())?;

        if result_verify {
            let token = Claims::create_token(&user)?;
            let result = json!({
                "username": user.username,
                "email": user.email,
                "role": user.role,
                "avatar": user.avatar,
                "token": token
            });
            GeneralResponse::ok_with_data(result)
        } else {
            let message = "Wrong password!".to_string();
            GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
        }
    } else {
        let message = "username not found!".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

use crate::model::{
    claim::Claims, database::{User, UserOutput, UserRole}, error::AppError, response::GeneralResponse,
};
use axum::{extract::State, http::StatusCode};
use postgrest::Postgrest;
use serde_json::json;
use std::sync::Arc;

#[utoipa::path(
    put,
    tag = "Account",
    path = "/account/customer-to-salon-user",
    security(("Authorization" = []))
)]
pub async fn customer_to_salon_user(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
) -> Result<GeneralResponse, AppError> {
    if claims.role != UserRole::Customer {
        let message = "This account is not customer role.".to_string();
        return GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message));
    }
    let query = db
        .from("users")
        .eq("id", claims.id.to_string())
        .update(r#"{"role": "SALON_USER"}"#)
        .single()
        .execute()
        .await?;
    if query.status().is_success() {
        let user: User = query.json().await?;
        let token = Claims::create_token(&user)?;
        let data = json!({
            "username": user.username,
            "fullName": user.full_name,
            "email": user.email,
            "role": user.role,
            "avatar": user.avatar,
            "token": token
        });
        GeneralResponse::ok_with_data(data)
    } else {
        GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, None)
    }
}

use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, HeaderMap},
    Json,
};
use axum_extra::extract::cookie::Cookie;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{
    claim::Claims,
    database::{User, UserGender},
    error::AppError,
    response::GeneralResponse,
};

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct SignupInput {
    username: String,
    password: String,
    full_name: Option<String>,
    email: Option<String>,
    gender: Option<UserGender>,
}

const SIGN_UP_QUERY: &str = "INSERT INTO
users(username, password, full_name, email, gender)
VALUES ($1, $2, $3, $4, $5) RETURNING *";

#[utoipa::path(post, tag = "Account", path = "/account/sign-up")]
pub async fn sign_up(
    State(db): State<Arc<Pool<Postgres>>>,
    Json(mut signup_input): Json<SignupInput>,
) -> Result<GeneralResponse, AppError> {
    let validate_user: Vec<User> =
        sqlx::query_as("SELECT date_of_birth::text,* FROM users where username = $1")
            .bind(signup_input.username.as_str())
            .fetch_all(db.as_ref())
            .await?;
    if !validate_user.is_empty() {
        return GeneralResponse::new_general("S0001");
    }

    //Hash password
    let password_hash = bcrypt::hash(signup_input.password, 4)?;
    signup_input.password = password_hash;

    let user: User = sqlx::query_as(SIGN_UP_QUERY)
        .bind(signup_input.username)
        .bind(signup_input.password)
        .bind(signup_input.full_name)
        .bind(signup_input.email)
        .bind(signup_input.gender)
        .fetch_one(db.as_ref())
        .await?;

    let token = Claims::create_token(&user)?;

    let cookie = Cookie::build(("token", &token))
        .path("/")
        .secure(true)
        .http_only(true);
    let mut header = HeaderMap::new();
    header.append(header::SET_COOKIE, cookie.to_string().parse()?);
    let data = json!({
        "username": user.username,
        "fullName": user.full_name,
        "email": user.email,
        "role": user.role,
        "avatar": user.avatar,
        "token": token
    });

    GeneralResponse::new(header, "G0001", data)
}

// ------------------------------------------------------------------------------------

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct SigninInput {
    username: String,
    password: String,
}

const SIGNIN_QUERY: &str = "SELECT users.*, date_of_birth::text
FROM users
WHERE username = $1";

#[utoipa::path(post, tag = "Account", path = "/account/sign-in")]
pub async fn sign_in(
    State(db): State<Arc<Pool<Postgres>>>,
    Json(signin_input): Json<SigninInput>,
) -> Result<GeneralResponse, AppError> {
    let user: User = match sqlx::query_as(SIGNIN_QUERY)
        .bind(signin_input.username)
        .fetch_one(db.as_ref())
        .await
    {
        Ok(user) => user,
        Err(_) => return GeneralResponse::new_general("S0002"),
    };
    let is_valid_password = bcrypt::verify(
        signin_input.password,
        &user.password.clone().unwrap_or_default(),
    )?;
    if !is_valid_password {
        return GeneralResponse::new_general("S0003");
    }
    let token = Claims::create_token(&user)?;

    let cookie = Cookie::build(("token", &token))
        .path("/")
        .secure(true)
        .http_only(true);
    let mut header = HeaderMap::new();
    header.append(header::SET_COOKIE, cookie.to_string().parse()?);
    let data = json!({
        "username": user.username,
        "fullName": user.full_name,
        "email": user.email,
        "role": user.role,
        "avatar": user.avatar,
        "token": token
    });

    GeneralResponse::new(header, "G0001", data)
}

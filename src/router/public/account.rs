use std::{sync::Arc, time::Duration};

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::cookie::{Cookie, Expiration};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{types::time::OffsetDateTime, Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{
    claim::{Claims, HOUR_TO_SECOND},
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
        sqlx::query_as("SELECT * FROM users where username = $1")
            .bind(signup_input.username.as_str())
            .fetch_all(db.as_ref())
            .await?;
    if !validate_user.is_empty() {
        let message = "Username already existed!".to_string();
        return GeneralResponse::new_error(message);
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

    GeneralResponse::new(StatusCode::OK, header, data)
}

// ------------------------------------------------------------------------------------

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct SigninInput {
    username: String,
    password: String,
}

const SIGNIN_QUERY: &str = "SELECT users.*
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
        Err(err) => return GeneralResponse::new_error(err.to_string()),
    };
    let is_valid_password = bcrypt::verify(
        signin_input.password,
        &user.password.clone().unwrap_or_default(),
    )?;
    if !is_valid_password {
        return GeneralResponse::new_general(StatusCode::UNAUTHORIZED);
    }
    let token = Claims::create_token(&user)?;

    let expires =
        Expiration::from(OffsetDateTime::now_utc() + Duration::from_secs(HOUR_TO_SECOND * 24 * 30));
    let cookie = Cookie::build(("token", &token))
        .path("/")
        .expires(expires)
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

    GeneralResponse::new(StatusCode::OK, header, data)
}

#[utoipa::path(delete, tag = "Account", path = "/account/sign-out")]
pub async fn sign_out() -> Result<GeneralResponse, AppError> {
    let expires = Expiration::from(OffsetDateTime::UNIX_EPOCH);
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .expires(expires)
        .secure(true)
        .http_only(true);
    let mut header = HeaderMap::new();
    header.append(header::SET_COOKIE, cookie.to_string().parse()?);

    GeneralResponse::new(StatusCode::OK, header, Option::<bool>::None)
}

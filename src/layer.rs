use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::extract::CookieJar;
use sqlx::{Pool, Postgres};

use crate::model::{
    claim::Claims,
    database::{User, UserRole},
    response::GeneralResponse,
};

const AUTH_USER_QUERY: &str = "SELECT *
FROM users
WHERE id = $1
AND username = $2
";

pub async fn authenticated_layer(
    State(db): State<Arc<Pool<Postgres>>>,
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Response {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        })
        .or_else(|| {
            cookie_jar
                .get("token")
                .map(|cookie| cookie.value().to_string())
        });

    let token = match token {
        Some(token) => token,
        None => return GeneralResponse::new_general(StatusCode::UNAUTHORIZED).into_response(),
    };
    let mut claims = match Claims::from_token(&token) {
        Ok(claims) => claims,
        Err(err) => return err.into_response(),
    };

    let user: User = match sqlx::query_as(AUTH_USER_QUERY)
        .bind(claims.id)
        .bind(claims.username.as_str())
        .fetch_one(db.as_ref())
        .await
    {
        Ok(result) => result,
        Err(_) => return GeneralResponse::new_general(StatusCode::UNAUTHORIZED).into_response(),
    };

    claims.role = user.role;

    req.extensions_mut().insert(claims);
    next.run(req).await
}

pub async fn admin_layer(
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Response {
    if claims.role == Some(UserRole::Admin) {
        next.run(req).await
    } else {
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED).into_response()
    }
}

pub async fn salon_owner_layer(
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Response {
    if claims.role == Some(UserRole::SalonOwner) {
        next.run(req).await
    } else {
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED).into_response()
    }
}

pub async fn customer_layer(
    Extension(claims): Extension<Claims>,
    req: Request,
    next: Next,
) -> Response {
    if claims.role == Some(UserRole::Customer) {
        next.run(req).await
    } else {
        GeneralResponse::new_general(StatusCode::UNAUTHORIZED).into_response()
    }
}

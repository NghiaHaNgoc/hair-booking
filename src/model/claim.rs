use std::{
    env,
    time::{self, Duration, SystemTime},
};

use super::{
    database::{User, UserRole},
    error::AppError,
    response::GeneralResponse,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub id: u64,
    pub username: String,
    pub role: UserRole,
    pub exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = GeneralResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            match parts.extract::<TypedHeader<Authorization<Bearer>>>().await {
                Ok(header) => header,
                Err(err) => {
                    let status = axum::http::StatusCode::UNAUTHORIZED;
                    let message = err.to_string();
                    let res = GeneralResponse::new_general(status, Some(message)).unwrap();
                    return Err(res);
                }
            };
        let secret_key = env::var("JWT_KEY").expect("JWT_KEY must be set!");

        // Decode the user data
        let token_data = match decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data,
            Err(err) => {
                let status = axum::http::StatusCode::BAD_REQUEST;
                let message = err.to_string();
                let res = GeneralResponse::new_general(status, Some(message)).unwrap();
                return Err(res);
            }
        };

        Ok(token_data.claims)
    }
}

const HOUR_TO_SECOND: u64 = 60 * 60;

impl Claims {
    pub fn create_token(user: &User) -> Result<String, AppError> {
        // Extract data from db
        let id = match user.id {
            Some(id) => id,
            None => return Err(AppError::new("id not found in db!".to_string())),
        };
        let username = match user.username.as_ref() {
            Some(username) => username.clone(),
            None => return Err(AppError::new("username not found in db!".to_string())),
        };
        let role = match user.role.as_ref() {
            Some(role) => role.clone(),
            None => return Err(AppError::new("role not found in db!".to_string())),
        };

        // Create time expired
        let now = SystemTime::now();
        let exp_after = Duration::from_secs(HOUR_TO_SECOND * 24 * 30);
        let exp = (now + exp_after)
            .duration_since(time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let jwt_key = env::var("JWT_KEY").expect("JWT_KEY must be set!");
        let claims = Claims {
            id,
            username,
            role,
            exp,
        };
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_key.as_bytes()),
        )?;
        Ok(token)
    }
}

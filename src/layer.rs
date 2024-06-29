use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use postgrest::Postgrest;

use crate::model::{claim::Claims, response::GeneralResponse};

pub async fn authenticated_layer(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
    req: Request,
    next: Next,
) -> Response {
    let query = match db
        .from("users")
        .select("id")
        .eq("id", claims.id.to_string())
        .eq("username", claims.username)
        .eq("role", claims.role.to_string())
        .single()
        .execute()
        .await
    {
        Ok(result) => result,
        Err(err) => {
            let message = format!("Err: {}", err);
            return GeneralResponse::new_general(StatusCode::INTERNAL_SERVER_ERROR, Some(message))
                .into_response();
        }
    };

    if !query.status().is_success() {
        return GeneralResponse::new_general(StatusCode::UNAUTHORIZED, None).into_response();
    }

    next.run(req).await
}

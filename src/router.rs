use std::sync::Arc;

use axum::{extract::DefaultBodyLimit, middleware, Router};
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;

use crate::layer;

mod admin;
mod customer;
mod general;
mod public;
mod salon;

const MB_TO_BYTE: usize = 1024 * 1024;

pub fn all_router(db: Arc<Postgrest>) -> Router {
    let public_router = public::public_router(db.clone());
    let authorization_router = authorization_router(db);
    Router::new()
        .merge(public_router)
        .merge(authorization_router)
        .layer(DefaultBodyLimit::max(MB_TO_BYTE * 10))
        .layer(CorsLayer::very_permissive())
}

fn authorization_router(db: Arc<Postgrest>) -> Router {
    let authenticated_layer =
        middleware::from_fn_with_state(db.clone(), layer::authenticated_layer);
    let general_router = general::general_router(db.clone());

    Router::new()
        .merge(general_router)
        .layer(authenticated_layer)
}

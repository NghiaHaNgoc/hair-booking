use std::sync::Arc;

use axum::{extract::DefaultBodyLimit, http::HeaderValue, middleware, Router};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;

use crate::layer;

//mod admin;
//mod customer;
pub mod general;
pub mod public;
pub mod salon_owner;

const MB_TO_BYTE: usize = 1024 * 1024;

pub fn all_router(db: Arc<Pool<Postgres>>) -> Router {
    let origins = [
        //HeaderValue::from_static("http://localhost:8080"),
        HeaderValue::from_static("http://localhost:5173"),
    ];
    let cors = CorsLayer::very_permissive()
        .allow_origin(origins)
        .allow_credentials(true);

    let public_router = public::public_router(db.clone());
    let authorization_router = authorization_router(db);
    Router::new()
        .merge(public_router)
        .merge(authorization_router)
        .layer(DefaultBodyLimit::max(MB_TO_BYTE * 10))
        .layer(cors)
}

fn authorization_router(db: Arc<Pool<Postgres>>) -> Router {
    let authenticated_layer =
        middleware::from_fn_with_state(db.clone(), layer::authenticated_layer);
    let general_router = general::general_router(db.clone());
    //let admin_router = admin::admin_router(db.clone());
    //let salon_user_router = salon_user::salon_user_router(db.clone());

    Router::new()
        .merge(general_router)
        //.nest("/admin", admin_router)
        //.nest("/salon-user", salon_user_router)
        .layer(authenticated_layer)
}

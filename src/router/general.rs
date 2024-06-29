use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;

use crate::service::account;

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/account/profile", get(account::get_profile))
        .with_state(db)
}

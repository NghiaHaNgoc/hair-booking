use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use postgrest::Postgrest;

use crate::{
    layer,
    service::{salon, user},
};

pub fn salon_user_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::salon_user_layer);
    Router::new()
        .route("/salon", get(salon::salon_user::list_salon))
        .route("/salon", post(salon::salon_user::create_salon))
        .route("/salon/:salon_id", delete(salon::salon_user::delete_salon))
        .with_state(db)
        .layer(layer)
}

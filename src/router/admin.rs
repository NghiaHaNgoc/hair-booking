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

pub fn admin_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        .route("/salon", post(salon::admin::create_salon))
        .route("/salon/:salon_id", delete(salon::admin::delete_salon))
        // User
        .route(
            "/user/salon/:salon_id",
            post(user::admin::create_salon_user),
        )
        .with_state(db)
        .layer(layer)
}

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
        // User
        .route("/user/salon-user", post(user::admin::create_salon_user))
        .route("/user/:user_id", delete(user::admin::delete_user))
        .route("/user", get(user::admin::list_user))
        .with_state(db)
        .layer(layer)
}

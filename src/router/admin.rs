use std::sync::Arc;

use axum::{
    middleware, routing::{get, post}, Router
};
use postgrest::Postgrest;

use crate::{layer, service::salon};

pub fn admin_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        .route("/admin/salon", post(salon::admin::create_salon))
        .with_state(db)
        .layer(layer)
}

use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use postgrest::Postgrest;

use crate::{layer, service::{salon, salon_bed}};

pub fn salon_user_router(db: Arc<Postgrest>) -> Router {
    let layer = middleware::from_fn(layer::salon_user_layer);
    Router::new()
        // Salon
        .route("/salon", get(salon::salon_user::list_salon_of_user))
        .route("/salon", post(salon::salon_user::create_salon))
        .route("/salon/:salon_id", put(salon::salon_user::update_salon))
        .route("/salon/:salon_id", delete(salon::salon_user::delete_salon))
        .route("/salon/:salon_id/media", post(salon::salon_user::create_salon_media))
        .route(
            "/salon/:salon_id/media/:media_id",
            delete(salon::salon_user::delete_salon_media),
        )
        // Salon bed
        .route("/salon/:salon_id/salon-bed", post(salon_bed::salon_user::create_salon_bed))
        .with_state(db)
        .layer(layer)
}

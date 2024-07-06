use std::sync::Arc;

use axum::{routing::get, Router};
use postgrest::Postgrest;

use crate::service::{account, salon_bed};

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/account/profile", get(account::get_profile))
        .route("/all-user/salon/:salon_id/available-salon-bed", get(salon_bed::all_user::list_available_salon_bed))
        .with_state(db)
}

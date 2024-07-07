use std::sync::Arc;

use axum::{routing::{get, post, put}, Router};
use postgrest::Postgrest;

use crate::service::{account, reservation, salon_bed};

pub fn general_router(db: Arc<Postgrest>) -> Router {
    Router::new()
        .route("/account/profile", get(account::get_profile))
        .route("/all-user/salon/:salon_id/available-salon-bed", get(salon_bed::all_user::list_available_salon_bed))
        .route("/all-user/reservation", post(reservation::all_user::create_reservation))
        .route("/all-user/reservation/:reservation_id/cancel", put(reservation::all_user::cancel_reservation))
        .route("/all-user/reservation", get(reservation::all_user::list_reservation_history))
        .with_state(db)
}

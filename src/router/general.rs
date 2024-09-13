use std::sync::Arc;

use axum::{routing::{get, post, put}, Router};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

use crate::model::api_doc::SecurityAddon;


mod account;

pub fn general_router(db: Arc<Pool<Postgres>>) -> Router {
    Router::new()
         .route("/account/profile", get(account::get_profile))
         .route("/account/profile", put(account::update_profile))
        // .route("/account/customer-to-salon-user", put(account::customer_to_salon_user))
        // .route("/all-user/salon/:salon_id/available-salon-bed", get(salon_bed::all_user::list_available_salon_bed))
        // .route("/all-user/reservation", post(reservation::all_user::create_reservation))
        // .route("/all-user/reservation/:reservation_id/cancel", put(reservation::all_user::cancel_reservation))
        // .route("/all-user/reservation", get(reservation::all_user::list_reservation_history))
        .with_state(db)
}

#[derive(OpenApi)]
#[openapi(
        paths(
        account::get_profile,
        account::update_profile,
        ),
        components(
            schemas(
            account::UpdateUserProfileInput,
        )
        ),
        modifiers(&SecurityAddon),
        //tags(
        //    (name = "Account", description = "")
        //)
    )]
pub struct GeneralApiDoc;

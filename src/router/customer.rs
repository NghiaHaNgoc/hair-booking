use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use reservation::AddReservationInput;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

use crate::{layer, model::api_doc::SecurityAddon};

mod reservation;

pub fn customer_router(db: Arc<Pool<Postgres>>) -> Router {
    let layer = middleware::from_fn(layer::customer_layer);
    Router::new()
        // Reservation
        .route("/reservation", post(reservation::add_reservation))
        .with_state(db)
        .layer(layer)
}

#[derive(OpenApi)]
#[openapi(
        paths(
        reservation::add_reservation
        ),
        components(
            schemas(
            AddReservationInput
        )
        ),
        modifiers(&SecurityAddon),
    )]
pub struct CustomerApiDoc;

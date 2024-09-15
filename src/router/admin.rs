use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

use crate::{layer, model::api_doc::SecurityAddon};

mod user;

pub fn admin_router(db: Arc<Pool<Postgres>>) -> Router {
    let layer = middleware::from_fn(layer::admin_layer);
    Router::new()
        // User
        //.route("/user/salon-user", post(user::admin::create_salon_user))
        // .route("/user/:user_id", delete(user::admin::delete_user))
        .route("/user", get(user::list_user))
        .route(
            "/customer-to-salon-owner/:user_id",
            put(user::customer_to_salon_owner),
        )
        .with_state(db)
        .layer(layer)
}

#[derive(OpenApi)]
#[openapi(
        paths(
        user::list_user,
        user::customer_to_salon_owner
        ),
        components(
            schemas(
        )
        ),
        modifiers(&SecurityAddon),
    )]
pub struct AdminApiDoc;

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use postgrest::Postgrest;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    model::api_doc,
    service::{account, salon},
};

pub fn public_router(db: Arc<Postgrest>) -> Router {
    let api_doc = api_doc::get_api_doc();
    Router::new()
        .merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api_doc))
        .route("/account/sign-in", post(account::sign_in))
        .route("/account/sign-up", post(account::sign_up))
        .route("/public/salon", get(salon::public::list_salon))
        .with_state(db)
}

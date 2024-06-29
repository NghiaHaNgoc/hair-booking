use std::sync::Arc;

use axum::{routing::post, Router};
use postgrest::Postgrest;
use utoipa_swagger_ui::SwaggerUi;

use crate::{model::api_doc, service::account};

pub fn public_router(db: Arc<Postgrest>) -> Router {
    let api_doc = api_doc::get_api_doc();
    Router::new()
        .merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api_doc))
        .route("/account/sign-in", post(account::sign_in))
        .with_state(db)
}

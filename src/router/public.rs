use std::sync::Arc;

use axum::{routing::{delete, get, post}, Router};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::model::{
    api_doc::{self, SecurityAddon},
    database,
};

mod account;
mod salon;

pub fn public_router(db: Arc<Pool<Postgres>>) -> Router {
    let api_doc = api_doc::get_api_doc();
    Router::new()
        .merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api_doc))
        .route("/account/sign-in", post(account::sign_in))
        .route("/account/sign-up", post(account::sign_up))
        .route("/account/sign-out", delete(account::sign_out))
         .route("/public/salon", get(salon::list_salon))
         .route("/public/salon/:salon_id", get(salon::salon_detail))
        // .route(
        //     "/public/salon/:salon_id/salon-bed",
        //     get(salon_bed::public::list_salon_bed),
        // )
        .with_state(db)
}

#[derive(OpenApi)]
#[openapi(
        paths(
        account::sign_in,
        account::sign_up,
        account::sign_out,
        salon::list_salon,
        salon::salon_detail
        ),
        components(
            schemas(
            account::SigninInput,
            account::SignupInput,
            database::UserGender,
            database::UserRole
        )
        ),
        modifiers(&SecurityAddon),
    )]
pub struct PublicApiDoc;

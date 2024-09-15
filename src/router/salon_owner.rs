use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use salon::UpdateSalonInput;
use salon_branch::AddSalonBranchInput;
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;

use crate::{layer, model::api_doc::SecurityAddon};

mod salon;
mod salon_branch;

pub fn salon_owner_router(db: Arc<Pool<Postgres>>) -> Router {
    let layer = middleware::from_fn(layer::salon_owner_layer);
    Router::new()
        // Salon
        .route("/salon", get(salon::get_salon))
        .route("/salon", put(salon::update_salon))
        .route("/salon/branch", post(salon_branch::add_branch))
        .route("/salon/branch/:id", delete(salon_branch::delete_branch))
        // .route("/salon/:salon_id", delete(salon::salon_user::delete_salon))
        // .route(
        //     "/salon/:salon_id/media",
        //     post(salon::salon_user::create_salon_media),
        // )
        // .route(
        //     "/salon/:salon_id/media/:media_id",
        //     delete(salon::salon_user::delete_salon_media),
        // )
        // // Salon bed
        // .route(
        //     "/salon/:salon_id/salon-bed",
        //     post(salon_bed::salon_user::create_salon_bed),
        // )
        // .route(
        //     "/salon-bed/:salon_bed_id",
        //     delete(salon_bed::salon_user::delete_salon_bed),
        // )
        .with_state(db)
        .layer(layer)
}

#[derive(OpenApi)]
#[openapi(
        paths(
        salon::get_salon,
        salon::update_salon,
        salon_branch::add_branch,
        salon_branch::delete_branch
        ),
        components(
            schemas(
            UpdateSalonInput,
            AddSalonBranchInput
        )
        ),
        modifiers(&SecurityAddon),
    )]
pub struct SalonOwnerApiDoc;

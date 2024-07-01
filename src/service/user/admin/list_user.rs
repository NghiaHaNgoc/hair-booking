use std::sync::Arc;

use axum::extract::{Query, State};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::IntoParams;

use crate::{
    model::{
        database::{UserOutput, UserRole},
        error::AppError,
        response::GeneralResponse,
    },
    utils,
};

#[derive(IntoParams, Serialize, Deserialize, Debug, Clone)]
pub struct UserQueryInput {
    role: Option<UserRole>,
    page: Option<usize>,
    limit: Option<usize>,
}

#[utoipa::path(
    get,
    tag = "User",
    path = "/admin/user",
    security(("Authorization" = [])),
    params(
        UserQueryInput
    ),
    responses(
        (status = 200, description = "List user")
    )
)]

pub async fn list_user(
    State(db): State<Arc<Postgrest>>,
    Query(query_params): Query<UserQueryInput>,
) -> Result<GeneralResponse, AppError> {
    let (page, limit) = utils::extract_page_and_limit(query_params.page, query_params.limit);
    let (from_index, to_index) = utils::get_query_from_to(page, limit)?;

    let mut query = db.from("users").select("*");
    if let Some(role) = query_params.role {
        query = query.eq("role", role.to_string());
    }
    let query = query
        .exact_count()
        .range(from_index as usize, to_index as usize)
        .order("id.asc")
        .execute()
        .await?;

    let total = utils::total_from_header(query.headers())?;
    let pages = utils::total_pages(total, limit);
    if query.status().is_success() {
        let salons: Vec<UserOutput> = query.json().await?;
        let data = json!({
            "users": salons,
            "pages": pages,
            "total": total
        });
        GeneralResponse::ok_with_data(data)
    } else {
        let salons: Vec<UserOutput> = Vec::new();
        let data = json!({
            "users": salons,
            "pages": pages,
            "total": total
        });
        GeneralResponse::ok_with_data(data)
    }
}

use std::sync::Arc;

use axum::
    extract::{Query, State}
;
use postgrest::Postgrest;
use serde_json::json;

use crate::{model::{
    database::{GeneralPagingQueryInput, GeneralUserOutput},
    error::AppError,
    response::GeneralResponse,
}, utils};

#[utoipa::path(
    get,
    tag = "User",
    path = "/admin/user",
    security(("Authorization" = [])),
    params(
        GeneralPagingQueryInput
    ),
    responses(
        (status = 200, description = "List user")
    )
)]
pub async fn list_user(
    State(db): State<Arc<Postgrest>>,
    Query(GeneralPagingQueryInput { page, limit }): Query<GeneralPagingQueryInput>,
) -> Result<GeneralResponse, AppError> {
    let (from_index, to_index) =
        utils::get_query_from_to(page.unwrap_or(1), limit.unwrap_or(9999))?;

    let query = db
        .from("users")
        .select("*")
        .exact_count()
        .range(from_index as usize, to_index as usize)
        .order("id.asc")
        .execute()
        .await?;

    let (range, total) = utils::range_and_total_from_header(query.headers())?;
    if query.status().is_success() {
        let salons: Vec<GeneralUserOutput> = query.json().await?;
        let data = json!({
            "users": salons,
            "range": range,
            "total": total
        });
        GeneralResponse::ok_with_data(data)
    } else {
        let salons: Vec<GeneralUserOutput> = Vec::new();
        let data = json!({
            "users": salons,
            "range": range,
            "total": total
        });
        GeneralResponse::ok_with_data(data)
    }
}

use std::sync::Arc;

use axum::extract::{Query, State};
use serde_json::json;
use sqlx::{FromRow, Pool, Postgres, Row};

use crate::{
    model::{
        database::{GeneralPagingQueryInput, UserOutput},
        error::AppError,
        response::GeneralResponse,
    },
    utils,
};

const LIST_USER_QUERY: &str = "
SELECT *, COUNT(*) OVER () as total
FROM users
ORDER BY id
OFFSET $1
LIMIT $2
";

#[utoipa::path(get,
    tag = "User",
    path = "/admin/user",
    params(
        GeneralPagingQueryInput
    ),
    responses(
        (status = 200, description = "Get list of users")
    )
)]
pub async fn list_user(
    State(db): State<Arc<Pool<Postgres>>>,
    Query(GeneralPagingQueryInput { page, limit }): Query<GeneralPagingQueryInput>,
) -> Result<GeneralResponse, AppError> {
    let (page, limit) = utils::extract_page_and_limit(page, limit);
    let offset = (page - 1) * limit;

    let users = sqlx::query(LIST_USER_QUERY)
        .bind(offset)
        .bind(limit)
        .fetch_all(db.as_ref())
        .await?;
    let mut total: Option<i64> = None;
    let users: Vec<UserOutput> = users
        .into_iter()
        .map(|user| {
            if total == None {
                total = user.try_get("total").ok();
            }
            let user = UserOutput::from_row(&user).unwrap_or_default();
            user
        })
        .collect();

    let total = total.unwrap_or(0);
    let pages = utils::total_pages(total, limit);

    let data = json!({
        "users": users,
        "pages": pages,
        "total": total
    });
    GeneralResponse::ok_with_data(data)
}

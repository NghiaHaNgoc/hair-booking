use std::sync::Arc;

use axum::extract::{Path, Query, State};
use serde_json::json;
use sqlx::{FromRow, Pool, Postgres, Row};

use crate::{
    model::{
        database::{GeneralPagingQueryInput, User, UserOutput, UserRole},
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

/// Get list of users
#[utoipa::path(
    get,
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
    Query(GeneralPagingQueryInput { offset, limit }): Query<GeneralPagingQueryInput>,
) -> Result<GeneralResponse, AppError> {

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

    let data = json!({
        "users": users,
        "total": total
    });
    GeneralResponse::ok_with_data(data)
}

// -------------------------------------------------------------------------

const CUSTOMER_TO_SALON_OWNER_QUERY: &str = "
with salon as (
insert INTO salons DEFAULT VALUES returning id
)
UPDATE users SET
salon_id = (SELECT id FROM salon),
role = 'SALON_OWNER'
WHERE users.id = $1
RETURNING *
";

/// Change role of an user from CUSTOMER to SALON_OWNER
#[utoipa::path(
    put,
    tag = "User",
    path = "/admin/customer-to-salon-owner/{id}",
)]
pub async fn customer_to_salon_owner(
    State(db): State<Arc<Pool<Postgres>>>,
    Path(user_id): Path<i64>,
) -> Result<GeneralResponse, AppError> {
    let validate_user: User = sqlx::query_as(
        "
SELECT * FROM users where id = $1
",
    )
    .bind(user_id)
    .fetch_one(db.as_ref())
    .await?;
    if let Some(role) = validate_user.role {
        if role != UserRole::Customer {
            return GeneralResponse::new_error("User role must be customer!".to_string());
        }
    }

    let user: UserOutput = sqlx::query_as(CUSTOMER_TO_SALON_OWNER_QUERY)
        .bind(user_id)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(user)
}

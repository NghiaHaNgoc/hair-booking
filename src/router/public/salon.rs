use std::sync::Arc;

use axum::extract::{Path, Query, State};
use serde_json::json;
use sqlx::{FromRow, Pool, Postgres, Row};

use crate::{
    model::{
        database::{GeneralPagingQueryInput, Salon, SalonDetailOutput},
        error::AppError,
        response::GeneralResponse,
    },
    utils,
};

const LIST_SALON_QUERY: &str = "SELECT *, COUNT(*) OVER () as total
FROM salons
ORDER BY id
OFFSET $1
LIMIT $2";

/// Get list of salon
#[utoipa::path(
    get,
    tag = "Salon",
    path = "/public/salon",
    params(GeneralPagingQueryInput)
)]
pub async fn list_salon(
    State(db): State<Arc<Pool<Postgres>>>,
    Query(GeneralPagingQueryInput { offset, limit }): Query<GeneralPagingQueryInput>,
) -> Result<GeneralResponse, AppError> {
    let salons = sqlx::query(LIST_SALON_QUERY)
        .bind(offset)
        .bind(limit)
        .fetch_all(db.as_ref())
        .await?;

    let mut total: Option<i64> = None;
    let salons: Vec<Salon> = salons
        .into_iter()
        .map(|salon| {
            if total == None {
                total = salon.try_get("total").ok();
            }
            let salon = Salon::from_row(&salon).unwrap_or_default();
            salon
        })
        .collect();

    let total = total.unwrap_or(0);

    let data = json!({
        "salons": salons,
        "total": total
    });
    GeneralResponse::ok_with_data(data)
}

// ------------------------------------------------------------

const SALON_DETAIL_QUERY: &str = "
select sl.*,
COALESCE(
  json_agg(DISTINCT br.*) FILTER (WHERE br.id IS NOT NULL),
  '[]'::json
) AS salon_branches,
COALESCE(
  json_agg(DISTINCT tp.*) FILTER (WHERE tp.id IS NOT NULL),
  '[]'::json
) AS therapies
FROM salons sl
LEFT JOIN salon_branches br ON sl.id = br.salon_id
LEFT JOIN therapies tp ON sl.id = tp.salon_id 
WHERE sl.id = $1
GROUP BY sl.id
";

/// Get salon detail
#[utoipa::path(get, tag = "Salon", path = "/public/salon/{salonId}")]
pub async fn salon_detail(
    State(db): State<Arc<Pool<Postgres>>>,
    Path(salon_id): Path<i64>,
) -> Result<GeneralResponse, AppError> {
    let salon: SalonDetailOutput = sqlx::query_as(SALON_DETAIL_QUERY)
        .bind(salon_id)
        .fetch_one(db.as_ref())
        .await?;
    GeneralResponse::ok_with_data(salon)
}

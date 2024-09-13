use std::sync::Arc;

use axum::extract::{Path, Query, State};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, Pool, Postgres, Row};

use crate::{
    model::{
        database::{GeneralPagingQueryInput, GeneralStatus, Salon, SalonBranch},
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

#[utoipa::path(
    get,
    tag = "Salon",
    path = "/public/salon",
    params(
        GeneralPagingQueryInput
    ),
    responses(
        (status = 200, description = "Get list of salon")
    )
)]
pub async fn list_salon(
    State(db): State<Arc<Pool<Postgres>>>,
    Query(GeneralPagingQueryInput { page, limit }): Query<GeneralPagingQueryInput>,
) -> Result<GeneralResponse, AppError> {
    let (page, limit) = utils::extract_page_and_limit(page, limit);
    let offset = (page - 1) * limit;

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
    let pages = utils::total_pages(total, limit);

    let data = json!({
        "salons": salons,
        "pages": pages,
        "total": total
    });
    GeneralResponse::ok_with_data(data)
}

// ------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct SalonDetailOutput {
    pub id: Option<i64>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub status: Option<GeneralStatus>,
    #[sqlx(json)]
    pub salon_branches: Vec<SalonBranch>,
    pub created_at: Option<DateTime<Utc>>,
}

const SALON_DETAIL_QUERY: &str = "
select sl.*,
COALESCE(
  json_agg(br) FILTER (WHERE br.id IS NOT NULL),
  '[]'::json
) AS salon_branchs
FROM salons sl
LEFT JOIN salon_branches br
ON sl.id = br.salon_id
WHERE sl.id = $1
GROUP BY sl.id
ORDER BY sl.id ASC";

#[utoipa::path(
    get,
    tag = "Salon",
    path = "/public/salon/{salonId}",
    responses(
        (status = 200, description = "Get salon detail")
    )
)]
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

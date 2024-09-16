use std::sync::Arc;

use axum::extract::{Path, State};
use sqlx::{Pool, Postgres};

use crate::model::{database::SalonBed, error::AppError, response::GeneralResponse};



const LIST_SALON_BED_QUERY: &str = "
SELECT * FROM salon_beds
WHERE branch_id = $1
";

/// List bed of a branch
#[utoipa::path(
    get,
    tag = "Salon bed",
    path = "/public/branch/{branchId}/salon-bed",
)]
pub async fn list_bed(
    State(db): State<Arc<Pool<Postgres>>>,
    Path(branch_id): Path<i64>
) -> Result<GeneralResponse, AppError> {
    let beds: Vec<SalonBed> = sqlx::query_as(LIST_SALON_BED_QUERY)
        .bind(branch_id)
        .fetch_all(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(beds)
}

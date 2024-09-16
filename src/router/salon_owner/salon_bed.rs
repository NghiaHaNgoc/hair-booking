use std::sync::Arc;

use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{
    claim::Claims,
    database::{SalonBed, SalonBranch},
    error::AppError,
    response::GeneralResponse,
};

const ADD_SALON_BED_QUERY: &str = "
INSERT INTO salon_beds (
  branch_id,
  name
)
SELECT 
    salon_branches.id,
    $1
FROM salon_branches
INNER JOIN users ON users.salon_id = salon_branches.salon_id
WHERE salon_branches.id = $2
AND users.id = $3
RETURNING *
";

#[derive(ToSchema, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct AddSalonBedInput {
    name: String,
}

/// Add bed to salon branch of salon owner
#[utoipa::path(
    post,
    tag = "Salon bed",
    path = "/salon-owner/branch/{branchId}/salon-bed",
    security(("Authorization" = [])),
)]
pub async fn add_bed(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Path(branch_id): Path<i64>,
    Json(input): Json<AddSalonBedInput>,
) -> Result<GeneralResponse, AppError> {
    let bed: SalonBed = sqlx::query_as(ADD_SALON_BED_QUERY)
        .bind(input.name)
        .bind(branch_id)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await
        .map_err(|_| anyhow!("Branch not found or not your branch!"))?;

    GeneralResponse::ok_with_data(bed)
}

// -------------------------------------------------------------------------

const DELETE_BED_QUERY: &str = "
DELETE FROM salon_beds
USING users, salon_branches
WHERE salon_beds.branch_id = salon_branches.id
AND users.salon_id = salon_branches.salon_id
AND salon_beds.id = $1
AND users.id = $2
RETURNING salon_beds.*
";

/// Delete salon bed of salon owner
#[utoipa::path(
    delete,
    tag = "Salon bed",
    path = "/salon-owner/branch/salon-bed/{bedId}",
    security(("Authorization" = [])),
)]
pub async fn delete_bed(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Path(bed_id): Path<i64>,
) -> Result<GeneralResponse, AppError> {
    let _branch: SalonBranch = sqlx::query_as(DELETE_BED_QUERY)
        .bind(bed_id)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await
        .map_err(|_| anyhow!("Bed not found or not your salon bed!"))?;

    GeneralResponse::new_general(StatusCode::OK)
}

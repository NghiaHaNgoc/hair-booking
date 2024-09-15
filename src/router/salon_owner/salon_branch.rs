use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{
    claim::Claims, database::SalonBranch, error::AppError, response::GeneralResponse,
};

const ADD_SALON_BRANCH_QUERY: &str = "
INSERT INTO salon_branches (
  salon_id,
  address
)
SELECT 
    users.salon_id,
    $1
FROM
    users
WHERE users.id = $2
RETURNING *
";

#[derive(ToSchema, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct AddSalonBranchInput {
    address: String,
}

#[utoipa::path(
    post,
    tag = "Salon branch",
    path = "/salon-owner/salon/branch",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Add salon branch by salon owner")
    )
)]
pub async fn add_branch(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Json(input): Json<AddSalonBranchInput>,
) -> Result<GeneralResponse, AppError> {
    let branch: SalonBranch = sqlx::query_as(ADD_SALON_BRANCH_QUERY)
        .bind(input.address)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(branch)
}

// -------------------------------------------------------------------------

const DELETE_BRANCH_QUERY: &str = "
DELETE FROM salon_branches
USING users
WHERE users.salon_id = salon_branches.salon_id
AND users.id = $1
AND salon_branches.id = $2
RETURNING *;
";

#[utoipa::path(
    delete,
    tag = "Salon branch",
    path = "/salon-owner/salon/branch/{id}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Delete salon branch by salon owner")
    )
)]
pub async fn delete_branch(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Path(branch_id): Path<i64>,
) -> Result<GeneralResponse, AppError> {
    let _branch: SalonBranch = sqlx::query_as(DELETE_BRANCH_QUERY)
        .bind(claims.id)
        .bind(branch_id)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::new_general(StatusCode::OK)
}

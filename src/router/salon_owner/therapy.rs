use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{claim::Claims, database::Therapy, error::AppError, response::GeneralResponse};

#[derive(ToSchema, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct AddAndUpdateTherapyInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub duration: Option<String>,
}

const ADD_THERAPY_QUERY: &str = "
INSERT INTO therapies (
salon_id,
name,
description,
price,
duration
) select salon_id, $1, $2, $3, $4
FROM users WHERE users.id = $5
RETURNING *
";

/// Add therapy to salon of salon owner
#[utoipa::path(
    post,
    tag = "Therapy",
    path = "/salon-owner/salon/therapy",
    security(("Authorization" = [])),
)]
pub async fn add_therapy(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Json(input): Json<AddAndUpdateTherapyInput>,
) -> Result<GeneralResponse, AppError> {
    let branch: Therapy = sqlx::query_as(ADD_THERAPY_QUERY)
        .bind(input.name)
        .bind(input.description)
        .bind(input.price)
        .bind(input.duration)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(branch)
}

// -------------------------------------------------

const UPDATE_THERAPY_QUERY: &str = "

UPDATE therapies SET
name = $1,
description = $2,
price = $3,
duration = $4
FROM users
WHERE users.id = $5
AND therapies.id = $6
AND therapies.id = users.salon_id
RETURNING therapies.*
";

/// Update therapy salon of salon owner
#[utoipa::path(
    put,
    tag = "Therapy",
    path = "/salon-owner/salon/therapy/{id}",
    security(("Authorization" = [])),
)]
pub async fn update_therapy(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Path(therapy_id): Path<i64>,
    Json(input): Json<AddAndUpdateTherapyInput>,
) -> Result<GeneralResponse, AppError> {
    let branch: Therapy = sqlx::query_as(UPDATE_THERAPY_QUERY)
        .bind(input.name)
        .bind(input.description)
        .bind(input.price)
        .bind(input.duration)
        .bind(claims.id)
        .bind(therapy_id)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(branch)
}

// -------------------------------------------------

const DELETE_THERAPY_QUERY: &str = "
DELETE FROM therapies
USING users
WHERE users.salon_id = therapies.salon_id
AND users.id = $1
AND therapies.id = $2
RETURNING *;
";

/// Delete therapy salon of salon owner
#[utoipa::path(
    delete,
    tag = "Therapy",
    path = "/salon-owner/salon/therapy/{id}",
    security(("Authorization" = [])),
)]
pub async fn delete_therapy(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Path(therapy_id): Path<i64>,
) -> Result<GeneralResponse, AppError> {
    let branch: Therapy = sqlx::query_as(DELETE_THERAPY_QUERY)
        .bind(claims.id)
        .bind(therapy_id)
        .fetch_one(db.as_ref())
        .await?;

    GeneralResponse::ok_with_data(branch)
}

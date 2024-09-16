use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{claim::Claims, database::Therapy, error::AppError, response::GeneralResponse};

#[derive(ToSchema, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct AddTherapyInput {
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
    Json(input): Json<AddTherapyInput>,
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

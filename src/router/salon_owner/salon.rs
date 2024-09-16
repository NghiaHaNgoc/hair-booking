use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{
    claim::Claims,
    database::{Salon, SalonDetailOutput},
    error::AppError,
    response::GeneralResponse,
};

const SALON_DETAIL_QUERY: &str = "
SELECT sl.*,
COALESCE(
  json_agg(DISTINCT br.*) FILTER (WHERE br.id IS NOT NULL),
  '[]'::json
) AS salon_branches,
COALESCE(
  json_agg(DISTINC tp.*) FILTER (WHERE tp.id IS NOT NULL),
  '[]'::json
) AS therpies
FROM salons sl
INNER JOIN users ur ON ur.salon_id = sl.id
LEFT JOIN salon_branches br ON sl.id = br.salon_id
LEFT JOIN therapies tp ON sl.id = tp.salon_id
WHERE ur.id = $1
GROUP BY sl.id
";

/// Get salon of this salon owner
#[utoipa::path(
    get,
    tag = "Salon",
    path = "/salon-owner/salon",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Update salon by salon owner")
    )
)]
pub async fn get_salon(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
) -> Result<GeneralResponse, AppError> {
    let salon: SalonDetailOutput = sqlx::query_as(SALON_DETAIL_QUERY)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await?;
    GeneralResponse::ok_with_data(salon)
}

// -----------------------------------------------------------------------------

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct UpdateSalonInput {
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    //pub salon_branches: Vec<UpdateSalonBranchInput>,
    //pub status: Option<GeneralStatus>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct UpdateSalonBranchInput {
    pub id: Option<i64>,
    pub address: Option<String>,
}

const UPDATE_SALON_QUERY: &str = "
UPDATE salons SET
logo = $1,
cover_photo = $2,
name = $3,
phone = $4,
email = $5,
description = $6
FROM users
WHERE users.id = $7 AND salons.id = users.salon_id
RETURNING salons.*
";

/// Update salons information
#[utoipa::path(
    put,
    tag = "Salon",
    path = "/salon-owner/salon",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Update salon by salon owner")
    )
)]
pub async fn update_salon(
    State(db): State<Arc<Pool<Postgres>>>,
    Extension(claims): Extension<Claims>,
    Json(update_salon_input): Json<UpdateSalonInput>,
) -> Result<GeneralResponse, AppError> {
    let salon: Salon = sqlx::query_as(UPDATE_SALON_QUERY)
        .bind(update_salon_input.logo)
        .bind(update_salon_input.cover_photo)
        .bind(update_salon_input.name)
        .bind(update_salon_input.phone)
        .bind(update_salon_input.email)
        .bind(update_salon_input.description)
        .bind(claims.id)
        .fetch_one(db.as_ref())
        .await?;
    GeneralResponse::ok_with_data(salon)
}

use std::sync::Arc;

use axum::{extract::State, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{claim::Claims, database::Salon, error::AppError, response::GeneralResponse};

const GET_SALON_QUERY: &str = "
SELECT salons.* 
FROM salons
INNER JOIN users
ON users.salon_id = salons.id
WHERE users.id = $1
";

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
    let salon: Salon = sqlx::query_as(GET_SALON_QUERY)
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

//const UPDATE_SALON_BRANCH_QUERY: &str = "
//UPDATE salon_branches set
//address = $1
//WHERE salon_id = $2
//";
//
//const CREATE_SALON_BRANCH_QUERY: &str = "
//INSERT INTO salon_branches (salon_id, address) VALUES
//($1, $2)
//";

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

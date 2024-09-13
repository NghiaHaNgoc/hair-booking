use std::sync::Arc;

use axum::{
    extract::{Path, State}, http::StatusCode, Extension, Json
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sqlx::{Pool, Postgres};
use utoipa::ToSchema;

use crate::model::{
    claim::Claims, database::{GeneralStatus, Salon}, error::AppError, response::GeneralResponse
};

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
#[sqlx(default)]
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

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone, sqlx::FromRow, sqlx::Type, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
#[sqlx(default)]
pub struct UpdateSalonBranchInput {
    pub id: Option<i64>,
    pub address: Option<String>
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
    path = "/salon-user/salon/{salonId}",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Update salon by salon user")
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

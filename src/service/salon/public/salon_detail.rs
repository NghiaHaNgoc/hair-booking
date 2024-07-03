use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};

use crate::model::{
    database::{GeneralStatus, SalonMediaOutput},
    error::AppError,
    response::GeneralResponse,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SalonDetailOuput {
    pub id: Option<u64>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Option<u64>,
    pub medias: Option<Vec<SalonMediaOutput>>,
}

#[utoipa::path(
    get,
    tag = "Salon",
    path = "/public/salon/{salonId}",
    responses(
        (status = 200, description = "Get list of salon")
    )
)]

pub async fn salon_detail(
    State(db): State<Arc<Postgrest>>,
    Path(salon_id): Path<u64>,
) -> Result<GeneralResponse, AppError> {
    let query = db
        .from("salons")
        .select("*, medias:salon_medias(*)")
        .eq("id", salon_id.to_string())
        .single()
        .execute()
        .await?;

    if query.status().is_success() {
        let data: SalonDetailOuput = query.json().await?;
        GeneralResponse::ok_with_data(data)
    } else {
        let message = "salon not found.".to_string();
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

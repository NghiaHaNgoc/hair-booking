use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

use crate::model::{
    claim::Claims,
    database::{MediaType, Salon},
    error::AppError,
    response::GeneralResponse,
};

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateSalonInput {
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub user_id: Option<u64>,
    pub medias: Option<Vec<CreateSalonMediaInput>>,
}

#[skip_serializing_none]
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[schema(rename_all = "camelCase")]
pub struct CreateSalonMediaInput {
    #[serde(skip_deserializing)]
    pub salon_id: Option<u64>,
    pub url: String,
    pub media_type: MediaType,
}

#[utoipa::path(
    post,
    tag = "Salon",
    path = "/salon-user/salon",
    security(("Authorization" = [])),
    responses(
        (status = 200, description = "Create salon by salon user")
    )
)]
pub async fn create_salon(
    State(db): State<Arc<Postgrest>>,
    claims: Claims,
    Json(mut create_salon_input): Json<CreateSalonInput>,
) -> Result<GeneralResponse, AppError> {
    create_salon_input.user_id = Some(claims.id);
    let salon_medias = create_salon_input.medias;
    create_salon_input.medias = None;

    let signup_input_json = serde_json::to_string(&create_salon_input)?;

    let query_salon = db
        .from("salons")
        .insert(signup_input_json)
        .single()
        .execute()
        .await?;

    if query_salon.status().is_success() {
        let salon: Salon = query_salon.json().await?;
        if let Some(mut salon_medias) = salon_medias {
            if !salon_medias.is_empty() {
                for salon_media in salon_medias.iter_mut() {
                    salon_media.salon_id = salon.id;
                }
                let salon_media_json = serde_json::to_string(&salon_medias)?;
                db.from("salon_medias")
                    .insert(salon_media_json)
                    .execute()
                    .await?;
            }
        }

        GeneralResponse::ok_with_data(salon)
    } else {
        let message = query_salon.text().await?;
        GeneralResponse::new_general(StatusCode::BAD_REQUEST, Some(message))
    }
}

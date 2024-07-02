use utoipa::OpenApi;

use crate::model::{
    api_doc::SecurityAddon,
    database::{GeneralStatus, MediaType},
};

use self::salon_user::{
    create_salon::{CreateSalonInput, CreateSalonMediaInput},
    update_salon::UpdateSalonInput,
};

pub mod admin;
pub mod public;
pub mod salon_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        salon_user::create_salon::create_salon,
        salon_user::delete_salon::delete_salon,
        salon_user::list_salon::list_salon,
        salon_user::delete_salon_media::delete_salon_media,
        salon_user::update_salon::update_salon,
        public::list_salon::list_salon
    ),
    components(
        schemas(
            CreateSalonInput,
            CreateSalonMediaInput,
            UpdateSalonInput,
            GeneralStatus,
            MediaType
        )
    ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Salon", description = "")
        )
    )]
pub struct SalonApiDoc;

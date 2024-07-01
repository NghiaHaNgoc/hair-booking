use utoipa::OpenApi;

use crate::model::{
    api_doc::SecurityAddon,
    database::{GeneralStatus, MediaType},
};

use self::salon_user::create_salon::{CreateSalonInput, CreateSalonMediaInput};

pub mod admin;
pub mod public;
pub mod salon_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        salon_user::create_salon::create_salon,
        salon_user::delete_salon::delete_salon,
        salon_user::list_salon::list_salon
        // public::list_salon::list_salon
    ),
    components(
        schemas(
            CreateSalonInput,
            CreateSalonMediaInput,
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

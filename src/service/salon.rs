use utoipa::OpenApi;

use crate::model::api_doc::SecurityAddon;
use admin::create_salon::CreateSalonInput;

pub mod admin;

#[derive(OpenApi)]
#[openapi(
    paths(
        admin::create_salon::create_salon
    ),
    components(
        schemas(
            CreateSalonInput
        )
    ),
        modifiers(&SecurityAddon),
        security(("Authorization" = [])),
        tags(
            (name = "Salon", description = "")
        )
    )]
pub struct SalonApiDoc;

use utoipa::OpenApi;

use crate::model::{api_doc::SecurityAddon, database::GeneralPagingQueryInput};
use admin::create_salon::CreateSalonInput;

pub mod admin;
pub mod public;

#[derive(OpenApi)]
#[openapi(
    paths(
        admin::create_salon::create_salon,
        admin::delete_salon::delete_salon,
        public::list_salon::list_salon
    ),
    components(
        schemas(
            CreateSalonInput,
        )
    ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Salon", description = "")
        )
    )]
pub struct SalonApiDoc;

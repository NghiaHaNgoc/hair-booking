use utoipa::OpenApi;

use crate::model::api_doc::SecurityAddon;

use self::salon_user::create_salon_bed::CreateSalonBedInput;

pub mod salon_user;
// pub mod all_user;
pub mod public;

#[derive(OpenApi)]
#[openapi(
    paths(
        salon_user::create_salon_bed::create_salon_bed,
        public::list_salon_bed::list_salon_bed
    ),
    components(
        schemas(
            CreateSalonBedInput
        )
    ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Salon bed", description = "")
        )
    )]
pub struct SalonBedApiDoc;

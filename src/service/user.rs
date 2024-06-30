use utoipa::OpenApi;

use crate::model::api_doc::SecurityAddon;

use self::admin::create_salon_user::CreateSalonUserInput;

pub mod admin;

#[derive(OpenApi)]
#[openapi(
    paths(
        admin::create_salon_user::create_salon_user,
        admin::delete_user::delete_user,
        // admin::list_user::list_user
    ),
    components(
        schemas(
            CreateSalonUserInput
        )
    ),
        modifiers(&SecurityAddon),
        tags(
            (name = "User", description = "")
        )
    )]
pub struct UserApiDoc;

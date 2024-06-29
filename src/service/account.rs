mod get_profile;
mod sign_in;

pub use get_profile::get_profile;
pub use sign_in::sign_in;
use utoipa::OpenApi;

use crate::model::api_doc::SecurityAddon;

#[derive(OpenApi)]
#[openapi(
        paths(
        sign_in::sign_in,
        get_profile::get_profile
        ),
        components(
            schemas(sign_in::LoginInput)
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Account", description = "")
        )
    )]
pub struct AccountApiDoc;

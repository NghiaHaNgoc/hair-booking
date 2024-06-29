mod get_profile;
mod sign_in;
mod sign_up;

pub use get_profile::get_profile;
pub use sign_in::sign_in;
pub use sign_up::sign_up;

use utoipa::OpenApi;

use crate::model::{api_doc::SecurityAddon, database};

#[derive(OpenApi)]
#[openapi(
        paths(
        sign_in::sign_in,
        get_profile::get_profile,
        sign_up::sign_up
        ),
        components(
            schemas(
            sign_in::LoginInput,
            sign_up::SignupInput,
            database::UserGender,
            database::UserRole
        )
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Account", description = "")
        )
    )]
pub struct AccountApiDoc;

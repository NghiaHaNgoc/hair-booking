mod get_profile;
mod sign_in;
mod sign_up;
mod update_profile;
mod customer_to_salon_user;

pub use get_profile::get_profile;
pub use update_profile::update_profile;
pub use sign_in::sign_in;
pub use sign_up::sign_up;
pub use customer_to_salon_user::customer_to_salon_user;

use utoipa::OpenApi;

use crate::model::{api_doc::SecurityAddon, database};


#[derive(OpenApi)]
#[openapi(
        paths(
        sign_in::sign_in,
        sign_up::sign_up,
        get_profile::get_profile,
        update_profile::update_profile,
        customer_to_salon_user::customer_to_salon_user
        ),
        components(
            schemas(
            sign_in::LoginInput,
            sign_up::SignupInput,
            database::UserGender,
            database::UserRole,
            update_profile::UpdateUserProfileInput
        )
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Account", description = "")
        )
    )]
pub struct AccountApiDoc;

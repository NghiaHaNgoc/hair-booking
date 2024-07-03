use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::service::{account, salon, salon_bed, user};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }
}

pub fn get_api_doc() -> openapi::OpenApi {
    let mut api_doc = account::AccountApiDoc::openapi();
    api_doc.merge(salon::SalonApiDoc::openapi());
    api_doc.merge(user::UserApiDoc::openapi());
    api_doc.merge(salon_bed::SalonBedApiDoc::openapi());
    api_doc
}

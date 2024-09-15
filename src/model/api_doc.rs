use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};
use uuid::Uuid;

use crate::router::{
    admin::AdminApiDoc, general::GeneralApiDoc, public::PublicApiDoc, salon_owner::SalonOwnerApiDoc,
};

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
    let mut api_doc = PublicApiDoc::openapi();
    api_doc.merge(GeneralApiDoc::openapi());
    api_doc.merge(SalonOwnerApiDoc::openapi());
    api_doc.merge(AdminApiDoc::openapi());

    for (_, j) in api_doc.paths.paths.iter_mut() {
        for (_, y) in j.operations.iter_mut() {
            y.operation_id = Some(Uuid::new_v4().to_string());
        }
    }
    api_doc
}

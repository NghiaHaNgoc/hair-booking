use utoipa::OpenApi;

use crate::model::api_doc::SecurityAddon;

use self::all_user::create_reservation::CreateReservationInput;

pub mod all_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        all_user::create_reservation::create_reservation,
        all_user::cancel_reservation::cancel_reservation,
        all_user::list_reservation_history::list_reservation_history
    ),
    components(
        schemas(
            CreateReservationInput
        )
    ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Salon bed", description = "")
        )
    )]
pub struct ReservationApiDoc;

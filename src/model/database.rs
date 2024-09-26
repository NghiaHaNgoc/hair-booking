use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct User {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub date_of_birth: Option<String>,
    pub salon_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct Salon {
    pub id: Option<i64>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, sqlx::Type, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct SalonBranch {
    pub id: Option<i64>,
    pub address: Option<String>,
    pub salon_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, sqlx::Type, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct Therapy {
    pub id: Option<i64>,
    pub salon_id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub duration: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, sqlx::Type, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct SalonBed {
    pub id: Option<i64>,
    pub branch_id: Option<i64>,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, sqlx::Type, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct Reservation {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub therapy_id: Option<i64>,
    pub salon_bed_id: Option<i64>,
    pub time_from: Option<DateTime<Utc>>,
    pub time_to: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub status: Option<ReservationStatus>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(IntoParams, Serialize, Deserialize, Debug, Clone)]
pub struct GeneralPagingQueryInput {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy, sqlx::Type)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
#[schema(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "user_gender", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserGender {
    Male,
    Female,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy, sqlx::Type)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
#[schema(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "user_role", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Admin,
    SalonOwner,
    Customer,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy, sqlx::Type)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
#[schema(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "general_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeneralStatus {
    Activate,
    Inactivate,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy, sqlx::Type)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
#[schema(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "reservation_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReservationStatus {
    Waiting,
    Done,
    Cancel,
}

impl fmt::Display for UserGender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let role = match self {
            UserRole::Admin => "ADMIN",
            UserRole::SalonOwner => "SALON_OWNER",
            UserRole::Customer => "CUSTOMER",
        };
        write!(f, "{}", role)
    }
}

//impl fmt::Display for GeneralStatus {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self)
//    }
//}
//
//impl fmt::Display for MediaType {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self)
//    }
//}
//
//impl fmt::Display for ReservationStatus {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "{:?}", self)
//    }
//}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct UserOutput {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub date_of_birth: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct SalonDetailOutput {
    pub id: Option<i64>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub status: Option<GeneralStatus>,
    #[sqlx(json)]
    pub salon_branches: Vec<SalonBranch>,
    #[sqlx(json)]
    pub therapies: Vec<Therapy>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, Default)]
#[serde(rename_all(serialize = "camelCase"))]
#[sqlx(default)]
pub struct ReservationOutput {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub therapy_id: Option<i64>,
    pub salon_bed_id: Option<i64>,
    pub time_from: Option<DateTime<Utc>>,
    pub time_to: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub status: Option<ReservationStatus>,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(json)]
    pub therapy: Option<Therapy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(json)]
    pub salon_bed: Option<SalonBed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(json)]
    pub salon: Option<Salon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[sqlx(json)]
    pub salon_branch: Option<SalonBranch>
}

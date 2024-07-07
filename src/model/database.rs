use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Salon {
    pub id: Option<u64>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SalonMedia {
    pub id: Option<u64>,
    pub salon_id: Option<u64>,
    pub url: Option<String>,
    pub media_type: Option<MediaType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SalonBed {
    pub id: Option<u64>,
    pub salon_id: Option<u64>,
    pub name: Option<String>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reservation {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub salon_bed_id: Option<u64>,
    pub time_from: Option<DateTime<Utc>>,
    pub time_to: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub status: Option<ReservationStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(IntoParams, Serialize, Deserialize, Debug, Clone)]
pub struct GeneralPagingQueryInput {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum UserGender {
    MALE,
    FEMALE,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
#[serde(rename_all(
    serialize = "SCREAMING_SNAKE_CASE",
    deserialize = "SCREAMING_SNAKE_CASE"
))]
#[schema(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Admin,
    SalonUser,
    Customer,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum GeneralStatus {
    ACTIVATE,
    INACTIVATE,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum MediaType {
    IMAGE,
    VIDEO,
}

#[derive(ToSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum ReservationStatus {
    WAITING,
    DONE,
    CANCEL
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
            UserRole::SalonUser => "SALON_USER",
            UserRole::Customer => "CUSTOMER",
        };
        write!(f, "{}", role)
    }
}

impl fmt::Display for GeneralStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ReservationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct UserOutput {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SalonOuput {
    pub id: Option<u64>,
    pub logo: Option<String>,
    pub cover_photo: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SalonMediaOutput {
    pub id: Option<u64>,
    pub salon_id: Option<u64>,
    pub url: Option<String>,
    pub media_type: Option<MediaType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct SalonBedOutput {
    pub id: Option<u64>,
    pub salon_id: Option<u64>,
    pub name: Option<String>,
    pub status: Option<GeneralStatus>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReservationOuput {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub salon_bed_id: Option<u64>,
    pub time_from: Option<DateTime<Utc>>,
    pub time_to: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub status: Option<ReservationStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

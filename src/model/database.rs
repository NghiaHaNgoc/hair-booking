use std::fmt;

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
    pub created_at: Option<String>,
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
    pub created_at: Option<String>,
    pub user_id: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SalonMedia {
    pub id: Option<u64>,
    pub salon_id: Option<u64>,
    pub url: Option<String>,
    pub media_type: Option<MediaType>,
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

#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
#[schema(rename_all = "camelCase")]
pub struct UserOutput {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub gender: Option<UserGender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub created_at: Option<String>,
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
    pub created_at: Option<String>,
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

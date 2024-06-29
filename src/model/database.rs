use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<Gender>,
    pub role: Option<UserRole>,
    pub avatar: Option<String>,
    pub created_at: Option<String>,
    pub salon_id: Option<u64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Gender {
    MALE,
    FEMALE,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum UserRole {
    ADMIN,
    SALON,
    CUSTOMER,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

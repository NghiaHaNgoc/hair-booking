use std::{collections::HashMap, sync::OnceLock};

use axum::http::StatusCode;

pub fn response_message() -> &'static HashMap<StatusCode, &'static str> {
    static HASHMAP: OnceLock<HashMap<StatusCode, &str>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(StatusCode::OK, "Successfully!");
        m.insert(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error!");
        m.insert(StatusCode::UNAUTHORIZED, "Unauthorized!");
        m.insert(StatusCode::BAD_REQUEST, "Bad request!");
        m
    })
}

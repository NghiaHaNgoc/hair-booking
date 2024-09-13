use std::{collections::HashMap, sync::OnceLock};

pub fn response_message() -> &'static HashMap<&'static str, &'static str> {
    static HASHMAP: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // General
        m.insert("G0001", "Successfully!");
        m.insert("G0002", "Internal server error!");
        m.insert("G0003", "Unauthorized!");
        m.insert("G0004", "Bad request!");
        m.insert("G0005", "Undefined status code!");
        // Specific
        m.insert("S0001", "Username already existed!");
        m.insert("S0002", "Username not found!");
        m.insert("S0003", "Invalid password!");
        m
    })
}

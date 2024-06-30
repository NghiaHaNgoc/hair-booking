use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupabaseError {
    pub code: String,
    pub details: Option<String>,
    pub hint: Option<String>,
    pub message: Option<String>,
}

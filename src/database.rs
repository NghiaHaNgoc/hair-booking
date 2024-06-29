use std::env;

use postgrest::Postgrest;

pub fn database_connection() -> Postgrest {
    let endpoint = env::var("HB_SUPABASE_ENDPOINT").expect("HB_SUPABASE_ENDPOINT should be set!");
    let api_key = env::var("HB_SUPABASE_API_KEY").expect("HB_SUPABASE_API_KEY should be set!");
    Postgrest::new(endpoint).insert_header("apikey", api_key)
}

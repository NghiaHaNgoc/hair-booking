use std::env;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn database_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = PgPoolOptions::new().connect(&url).await?;
    Ok(pool)
}

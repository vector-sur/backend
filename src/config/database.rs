use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::time::Duration;

/// Creates and returns a MySQL connection pool.
pub async fn create_pool() -> Result<MySqlPool, sqlx::Error> {
    // Load the database URL from the .env file.
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be defined in the .env file");

    // Load the maximum number of connections from the .env file.
    let max_connections = std::env::var("MAX_CONNECTIONS")
        .expect("MAX_CONNECTIONS must be defined in the .env file")
        .parse::<u32>()
        .expect("MAX_CONNECTIONS must be a valid number");

    // Build the pool with a 5‑second acquire timeout.
    let pool = MySqlPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    Ok(pool)
}

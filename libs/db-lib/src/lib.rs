use sqlx::postgres::PgPoolOptions;

// Re-export PgPool so consumers don't need a direct sqlx dependency.
pub use sqlx::PgPool;

/// Creates and returns a PostgreSQL connection pool.
///
/// # Errors
/// Returns an error string if the pool cannot be established.
pub async fn create_pg_pool(database_url: &str) -> Result<PgPool, String> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {e}"))
}

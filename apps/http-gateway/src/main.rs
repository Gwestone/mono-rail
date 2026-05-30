mod app_state;
mod di;
mod http_router;

use config_lib::AppConfig;
use db_lib::create_pg_pool;
use di::build_app_state;
use http_router::http_router;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Load configuration from environment / .env file
    let config = AppConfig::load()?;

    // Initialise structured logging
    tracing_subscriber::fmt::init();

    // Create the database connection pool
    let pool = create_pg_pool(&config.database_url).await?;

    // Wire dependencies
    let state = build_app_state(pool, config);

    // Build the fully-wired router (state is consumed inside)
    let app = http_router(state.clone());

    // Start the server
    let listener = tokio::net::TcpListener::bind(state.config().server_url.clone())
        .await
        .map_err(|e| format!("Failed to bind: {e}"))?;

    tracing::info!("Listening on {}", state.config().server_url);

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server error: {e}"))
}

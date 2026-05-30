use auth_lib::auth_router;
use axum::{routing::get, Router};

/// Builds the main application router, merging all sub-routers.
pub fn http_router() -> Router {
    Router::new()
        .route("/health", get(get_health))
        .merge(auth_router())
}

async fn get_health() -> &'static str {
    "OK"
}

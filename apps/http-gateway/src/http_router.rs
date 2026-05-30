use auth_lib::{auth_router, AuthAppState};
use axum::{extract::FromRef, routing::get, Router};

use crate::app_state::AppState;

/// Builds the main application router with all sub-routers merged.
///
/// Sub-routers that use a narrower state slice (e.g. `AuthAppState`) are
/// converted to `Router<AppState>` via `.with_state()` before merging,
/// which axum resolves using the `FromRef<AppState>` implementations.
pub fn http_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(get_health))
        .merge(auth_router().with_state(AuthAppState::from_ref(&state)))
}

async fn get_health() -> &'static str {
    "OK"
}

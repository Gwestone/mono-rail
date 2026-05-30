use std::sync::Arc;

use axum::extract::FromRef;
use config_lib::AppConfig;
use db_lib::PgPool;

use auth_lib::AuthAppState;

/// Full application state for the HTTP gateway.
///
/// Cheap to clone — all heavy data lives behind the inner `Arc`.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    pub db: PgPool,
    pub config: AppConfig,
    pub adapters: Adapters,
}

struct Adapters {
    pub auth: AuthAppState,
}

impl AppState {
    pub fn new(db: PgPool, config: AppConfig, auth: AuthAppState) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                db,
                config,
                adapters: Adapters { auth },
            }),
        }
    }

    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }

    pub fn config(&self) -> &AppConfig {
        &self.inner.config
    }
}

/// Allows axum to extract `AuthAppState` directly from `AppState` in handlers.
impl FromRef<AppState> for AuthAppState {
    fn from_ref(state: &AppState) -> Self {
        state.inner.adapters.auth.clone()
    }
}

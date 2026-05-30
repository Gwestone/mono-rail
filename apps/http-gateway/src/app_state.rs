use std::sync::Arc;

use axum::extract::FromRef;
use config_lib::AppConfig;

use auth_lib::AuthAppState;

/// Full application state for the HTTP gateway.
///
/// Cheap to clone — all heavy data lives behind the inner `Arc`.
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    pub config: AppConfig,
    pub adapters: Adapters,
}

struct Adapters {
    pub auth: AuthAppState,
}

impl AppState {
    /// Creates a new instance of [`AppState`].
    pub fn new(config: AppConfig, auth: AuthAppState) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                config,
                adapters: Adapters { auth },
            }),
        }
    }

    /// Accesses the application configuration.
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

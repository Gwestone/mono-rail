use std::sync::Arc;

use crate::domain::ports::{CryptoPort, GetUserPort, TokenPort};

/// Slice of application state consumed by the auth sub-router.
///
/// The gateway's full `AppState` implements `axum::extract::FromRef<AppState>`
/// for this type, so axum can extract it directly in handlers.
#[derive(Clone)]
pub struct AuthAppState {
    pub get_user: Arc<dyn GetUserPort>,
    pub crypto: Arc<dyn CryptoPort>,
    pub token: Arc<dyn TokenPort>,
}

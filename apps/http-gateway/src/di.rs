use std::sync::Arc;

use auth_lib::{AuthAppState, BcryptCryptoService, JwtTokenService, PostgresUserRepository};
use config_lib::AppConfig;
use db_lib::PgPool;

use crate::app_state::AppState;

/// Wires all concrete implementations together and returns the application state.
pub fn build_app_state(pool: PgPool, config: AppConfig) -> AppState {
    let get_user = Arc::new(PostgresUserRepository::new(pool.clone()));
    let crypto = Arc::new(BcryptCryptoService);
    let token = Arc::new(JwtTokenService::new(
        config.jwt_secret.clone(),
        config.jwt_expires_in_seconds,
    ));

    let auth_state = AuthAppState { get_user, crypto, token };

    AppState::new(pool, config, auth_state)
}

use auth_lib::AuthAppState;
use config_lib::AppConfig;
use grpc_lib::{connect_grpc, AuthServiceClient};

use crate::app_state::AppState;

/// Wires the gRPC client connection and returns the AppState.
pub async fn build_app_state(config: AppConfig) -> Result<AppState, String> {
    let auth_service_url = std::env::var("AUTH_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:50051".to_string());

    // Connect to the gRPC auth-service (lazily)
    let channel = connect_grpc(auth_service_url)?;
    let auth_client = AuthServiceClient::new(channel);

    let auth_state = AuthAppState { auth_client };

    Ok(AppState::new(config, auth_state))
}

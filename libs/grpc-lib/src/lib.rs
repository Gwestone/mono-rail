pub mod generated;

pub use generated::auth::v1::{
    auth_service_client::AuthServiceClient,
    auth_service_server::{AuthService, AuthServiceServer},
    RegisterRequest, RegisterResponse, LoginRequest, LoginResponse,
};

use tonic::transport::{Channel, Endpoint};

/// Establishes a gRPC channel lazily to the specified address.
///
/// Connection attempts are deferred until the channel is first used.
/// This prevents startup race conditions in microservice architectures.
///
/// # Errors
/// Returns an error string if the address is malformed.
pub fn connect_grpc(address: String) -> Result<Channel, String> {
    Endpoint::from_shared(address)
        .map_err(|e| format!("Invalid gRPC address: {e}"))
        .map(|e| e.connect_lazy())
}

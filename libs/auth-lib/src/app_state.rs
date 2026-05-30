use grpc_lib::AuthServiceClient;
use tonic::transport::Channel;

/// Slice of application state consumed by the auth sub-router.
///
/// The gateway's full `AppState` implements `axum::extract::FromRef<AppState>`
/// for this type, so axum can extract it directly in handlers.
#[derive(Clone)]
pub struct AuthAppState {
    pub auth_client: AuthServiceClient<Channel>,
}

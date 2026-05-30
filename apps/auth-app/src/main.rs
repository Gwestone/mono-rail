mod api;

use std::net::SocketAddr;
use std::sync::Arc;

use api::grpc::auth_controller::GrpcAuthController;
use auth_lib::persistence::{BcryptCryptoService, JwtTokenService};
use config_lib::AppConfig;
use db_lib::create_pg_pool;
use grpc_lib::generated::auth::v1::auth_service_server::AuthServiceServer;
use user_lib::PostgresUserRepository;

#[tokio::main]
async fn main() -> Result<(), String> {
    // 1. Load config
    let config = AppConfig::load()?;

    // 2. Connect to database
    let pool = create_pg_pool(&config.database_url).await?;

    // 3. Instantiate concrete adapters (Hexagonal Persistence layer)
    let user_repository = Arc::new(PostgresUserRepository::new(pool));
    let crypto = Arc::new(BcryptCryptoService);
    let token = Arc::new(JwtTokenService::new(
        config.jwt_secret.clone(),
        config.jwt_expires_in_seconds,
    ));

    // 4. Instantiate and wire the App/API Controller
    let controller = GrpcAuthController {
        user_repository,
        crypto,
        token,
    };

    // 5. Start gRPC server
    let grpc_port = std::env::var("GRPC_PORT")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .map_err(|e| format!("Failed to parse GRPC_PORT: {e}"))?;

    let addr: SocketAddr = format!("0.0.0.0:{grpc_port}")
        .parse()
        .map_err(|e| format!("Failed to parse socket address: {e}"))?;

    println!("Starting auth-service gRPC server on {addr}...");

    tonic::transport::Server::builder()
        .add_service(AuthServiceServer::new(controller))
        .serve(addr)
        .await
        .map_err(|e| format!("gRPC Server error: {e}"))?;

    Ok(())
}

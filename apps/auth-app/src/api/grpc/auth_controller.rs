use std::sync::Arc;
use tonic::{Request, Response, Status};

use grpc_lib::{
    AuthService, RegisterRequest, RegisterResponse, LoginRequest, LoginResponse,
};
use auth_lib::domain::{
    LoginUserInput, LoginUserUseCase, RegisterUserInput, RegisterUserUseCase,
};
use user_lib::UserRepository;
use auth_lib::domain::ports::{CryptoPort, TokenPort};

/// gRPC controller for handling authorization requests.
///
/// Adheres to Hexagonal Architecture as part of the App/API layer.
pub struct GrpcAuthController {
    pub user_repository: Arc<dyn UserRepository>,
    pub crypto: Arc<dyn CryptoPort>,
    pub token: Arc<dyn TokenPort>,
}

#[tonic::async_trait]
impl AuthService for GrpcAuthController {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        let use_case = RegisterUserUseCase {
            user_repository: self.user_repository.clone(),
            crypto: self.crypto.clone(),
        };

        match use_case
            .execute(RegisterUserInput {
                username: req.username,
                email: req.email,
                password: req.password,
            })
            .await
        {
            Ok(res) => Ok(Response::new(RegisterResponse {
                user_id: res.user_id.to_string(),
                username: res.username,
                email: res.email,
                error: String::new(),
            })),
            Err(e) => Ok(Response::new(RegisterResponse {
                user_id: String::new(),
                username: String::new(),
                email: String::new(),
                error: e,
            })),
        }
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let use_case = LoginUserUseCase {
            user_repository: self.user_repository.clone(),
            crypto: self.crypto.clone(),
            token: self.token.clone(),
        };

        match use_case
            .execute(LoginUserInput {
                email: req.email,
                password: req.password,
            })
            .await
        {
            Ok(res) => Ok(Response::new(LoginResponse {
                access_token: res.access_token,
                error: String::new(),
            })),
            Err(e) => Ok(Response::new(LoginResponse {
                access_token: String::new(),
                error: e,
            })),
        }
    }
}

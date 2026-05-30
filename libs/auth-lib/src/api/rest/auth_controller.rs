use axum::{http::StatusCode, routing::post, Json, Router};
use serde::Serialize;

use crate::domain::{LoginUserDto, RegisterUserDto};

/// Builds the auth sub-router mounted at `/auth`.
pub fn auth_router() -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

/// POST /auth/register
async fn register(
    Json(payload): Json<RegisterUserDto>,
) -> (StatusCode, Json<AuthResponse>) {
    // TODO: delegate to use-case / service layer
    let response = AuthResponse {
        message: format!("User '{}' registered", payload.username),
    };
    (StatusCode::CREATED, Json(response))
}

/// POST /auth/login
async fn login(
    Json(payload): Json<LoginUserDto>,
) -> (StatusCode, Json<AuthResponse>) {
    // TODO: delegate to use-case / service layer
    let response = AuthResponse {
        message: format!("User '{}' logged in", payload.email),
    };
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
struct AuthResponse {
    message: String,
}

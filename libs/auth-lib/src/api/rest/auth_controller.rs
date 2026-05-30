use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::app_state::AuthAppState;

/// Builds the auth sub-router. Requires [`AuthAppState`] as axum state.
pub fn auth_router() -> Router<AuthAppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

/// POST /auth/register
async fn register(
    State(state): State<AuthAppState>,
    Json(payload): Json<RegisterPayload>,
) -> (StatusCode, Json<RegisterResponse>) {
    let mut client = state.auth_client.clone();

    let request = tonic::Request::new(grpc_lib::RegisterRequest {
        username: payload.username,
        email: payload.email,
        password: payload.password,
    });

    match client.register(request).await {
        Ok(response) => {
            let res = response.into_inner();
            if !res.error.is_empty() {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(RegisterResponse {
                        user_id: String::new(),
                        username: String::new(),
                        email: res.error,
                    }),
                )
            } else {
                (
                    StatusCode::CREATED,
                    Json(RegisterResponse {
                        user_id: res.user_id,
                        username: res.username,
                        email: res.email,
                    }),
                )
            }
        }
        Err(status) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RegisterResponse {
                user_id: String::new(),
                username: String::new(),
                email: format!("gRPC connection error: {}", status.message()),
            }),
        ),
    }
}

/// POST /auth/login
async fn login(
    State(state): State<AuthAppState>,
    Json(payload): Json<LoginPayload>,
) -> (StatusCode, Json<LoginResponse>) {
    let mut client = state.auth_client.clone();

    let request = tonic::Request::new(grpc_lib::LoginRequest {
        email: payload.email,
        password: payload.password,
    });

    match client.login(request).await {
        Ok(response) => {
            let res = response.into_inner();
            if !res.error.is_empty() {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(LoginResponse::error(res.error)),
                )
            } else {
                (
                    StatusCode::OK,
                    Json(LoginResponse::success(res.access_token)),
                )
            }
        }
        Err(status) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginResponse::error(format!("gRPC connection error: {}", status.message()))),
        ),
    }
}

// ─── Request / response schemas ──────────────────────────────────────────────

#[derive(Deserialize)]
struct RegisterPayload {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    user_id: String,
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    access_token: Option<String>,
    error: Option<String>,
}

impl LoginResponse {
    fn success(token: String) -> Self {
        Self {
            access_token: Some(token),
            error: None,
        }
    }

    fn error(message: String) -> Self {
        Self {
            access_token: None,
            error: Some(message),
        }
    }
}

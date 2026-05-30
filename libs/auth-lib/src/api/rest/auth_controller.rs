use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::app_state::AuthAppState;
use crate::domain::{LoginUserInput, LoginUserUseCase, RegisterUserInput, RegisterUserUseCase};

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
    let use_case = RegisterUserUseCase {
        user_repository: state.user_repository.clone(),
        crypto: state.crypto.clone(),
    };

    match use_case
        .execute(RegisterUserInput {
            username: payload.username,
            email: payload.email,
            password: payload.password,
        })
        .await
    {
        Ok(result) => (
            StatusCode::CREATED,
            Json(RegisterResponse {
                user_id: result.user_id.to_string(),
                username: result.username,
                email: result.email,
            }),
        ),
        Err(message) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(RegisterResponse {
                user_id: String::new(),
                username: String::new(),
                email: message,
            }),
        ),
    }
}

/// POST /auth/login
async fn login(
    State(state): State<AuthAppState>,
    Json(payload): Json<LoginPayload>,
) -> (StatusCode, Json<LoginResponse>) {
    let use_case = LoginUserUseCase {
        user_repository: state.user_repository.clone(),
        crypto: state.crypto.clone(),
        token: state.token.clone(),
    };

    match use_case
        .execute(LoginUserInput {
            email: payload.email,
            password: payload.password,
        })
        .await
    {
        Ok(result) => (
            StatusCode::OK,
            Json(LoginResponse::success(result.access_token)),
        ),
        Err(message) => (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse::error(message)),
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

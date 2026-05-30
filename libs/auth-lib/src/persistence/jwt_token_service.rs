use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::ports::TokenPort;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// JWT-backed implementation of [`TokenPort`].
pub struct JwtTokenService {
    secret: String,
    /// Token lifetime in seconds.
    expires_in_seconds: usize,
}

impl JwtTokenService {
    pub fn new(secret: String, expires_in_seconds: usize) -> Self {
        Self { secret, expires_in_seconds }
    }
}

impl TokenPort for JwtTokenService {
    fn generate_token(&self, user_id: Uuid) -> Result<String, String> {
        let expiry = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Time error: {e}"))?
            .as_secs() as usize
            + self.expires_in_seconds;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiry,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| format!("JWT error: {e}"))
    }
}

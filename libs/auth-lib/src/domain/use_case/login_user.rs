use std::sync::Arc;
use user_lib::UserRepository;

use crate::domain::ports::{CryptoPort, TokenPort};

/// Response returned after a successful login.
#[derive(Debug)]
pub struct LoginUserResponse {
    pub access_token: String,
}

/// Input for the login use-case.
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
}

/// Authenticates an existing user and returns a signed JWT.
pub struct LoginUserUseCase {
    pub user_repository: Arc<dyn UserRepository>,
    pub crypto: Arc<dyn CryptoPort>,
    pub token: Arc<dyn TokenPort>,
}

impl LoginUserUseCase {
    /// Validates credentials and generates an access token.
    ///
    /// # Errors
    /// Returns an error string on invalid credentials or token generation failure.
    pub async fn execute(
        &self,
        input: LoginUserInput,
    ) -> Result<LoginUserResponse, String> {
        let user = self
            .user_repository
            .find_by_email(&input.email)
            .await?
            .ok_or_else(|| "Invalid credentials".to_string())?;

        let is_valid = self.crypto.verify_password(&input.password, &user.password_hash);
        if !is_valid {
            return Err("Invalid credentials".to_string());
        }

        let access_token = self.token.generate_token(user.id)?;

        Ok(LoginUserResponse { access_token })
    }
}

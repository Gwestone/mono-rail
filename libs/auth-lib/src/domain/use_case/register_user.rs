use std::sync::Arc;

use uuid::Uuid;

use crate::domain::entity::User;
use crate::domain::ports::{CryptoPort, GetUserPort};

/// Response returned after a successful registration.
#[derive(Debug)]
pub struct RegisterUserResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
}

/// Input for the register use-case.
pub struct RegisterUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Creates a new user account.
pub struct RegisterUserUseCase {
    pub get_user: Arc<dyn GetUserPort>,
    pub crypto: Arc<dyn CryptoPort>,
}

impl RegisterUserUseCase {
    /// Registers a new user.
    ///
    /// # Errors
    /// Returns an error string if the email is taken or hashing fails.
    pub async fn execute(
        &self,
        input: RegisterUserInput,
    ) -> Result<RegisterUserResponse, String> {
        let already_exists = self.get_user.exists_by_email(&input.email).await?;
        if already_exists {
            return Err(format!("Email '{}' is already registered", input.email));
        }

        let password_hash = self.crypto.hash_password(&input.password)?;
        let user = User {
            id: Uuid::new_v4(),
            username: input.username,
            email: input.email,
            password_hash,
        };

        let saved = self.get_user.save(user).await?;

        Ok(RegisterUserResponse {
            user_id: saved.id,
            username: saved.username,
            email: saved.email,
        })
    }
}

use async_trait::async_trait;

use crate::domain::entity::User;

/// Port for persisting and retrieving users.
#[async_trait]
pub trait GetUserPort: Send + Sync {
    /// Finds a user by their email address.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;

    /// Persists a new user, returning the saved entity.
    async fn save(&self, user: User) -> Result<User, String>;

    /// Checks whether a user with the given email already exists.
    async fn exists_by_email(&self, email: &str) -> Result<bool, String>;
}

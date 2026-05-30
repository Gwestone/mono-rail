use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entity::User;

/// Port defining database operations for the User domain.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a new user in the database.
    async fn create(&self, user: User) -> Result<User, String>;

    /// Finds a user by their unique ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;

    /// Finds a user by their email address.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;

    /// Checks whether a user with the given email exists.
    async fn exists_by_email(&self, email: &str) -> Result<bool, String>;

    /// Updates an existing user's details.
    async fn update(&self, user: User) -> Result<User, String>;

    /// Deletes a user from the database by ID.
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}

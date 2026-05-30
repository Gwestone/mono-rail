use uuid::Uuid;

/// Port for generating authentication tokens.
pub trait TokenPort: Send + Sync {
    /// Generates a signed JWT for the given user ID.
    fn generate_token(&self, user_id: Uuid) -> Result<String, String>;
}

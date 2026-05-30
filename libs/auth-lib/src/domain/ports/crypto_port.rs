/// Port for password hashing and verification.
pub trait CryptoPort: Send + Sync {
    /// Hashes a plain-text password.
    fn hash_password(&self, plain: &str) -> Result<String, String>;

    /// Returns true if `plain` matches the stored `hash`.
    fn verify_password(&self, plain: &str, hash: &str) -> bool;
}

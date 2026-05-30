use bcrypt::{hash, verify, DEFAULT_COST};

use crate::domain::ports::CryptoPort;

/// bcrypt-backed implementation of [`CryptoPort`].
pub struct BcryptCryptoService;

impl CryptoPort for BcryptCryptoService {
    fn hash_password(&self, plain: &str) -> Result<String, String> {
        hash(plain, DEFAULT_COST).map_err(|e| format!("Hash error: {e}"))
    }

    fn verify_password(&self, plain: &str, stored_hash: &str) -> bool {
        verify(plain, stored_hash).unwrap_or(false)
    }
}

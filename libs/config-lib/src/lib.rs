use dotenvy::dotenv;

pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub server_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in_seconds: usize,
}

impl AppConfig {
    /// Loads configuration from environment variables (with `.env` fallback).
    ///
    /// # Errors
    /// Returns an error string for any missing or malformed required variable.
    pub fn load() -> Result<Self, String> {
        dotenv().ok();

        let port = std::env::var("PORT")
            .map_err(|_| "Missing required env var: PORT".to_string())?
            .parse::<u16>()
            .map_err(|e| format!("Failed to parse PORT: {e}"))?;

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/mydb".to_string());

        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| "Missing required env var: JWT_SECRET".to_string())?;

        let jwt_expires_in_seconds = std::env::var("JWT_EXPIRES_IN_SECONDS")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<usize>()
            .map_err(|e| format!("Failed to parse JWT_EXPIRES_IN_SECONDS: {e}"))?;

        Ok(Self {
            port,
            database_url,
            server_url: format!("0.0.0.0:{port}"),
            jwt_secret,
            jwt_expires_in_seconds,
        })
    }
}

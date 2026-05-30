pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub server_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let port = std::env::var("PORT")
            .map_err(|e| format!("Missing PORT: {e}"))?
            .parse::<u16>()
            .map_err(|e| format!("Failed to parse PORT: {e}"))?;

        Ok(Self {
            port: port,
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/mydb".to_string()),
            server_url: format!("0.0.0.0:{0}", port),
        })
    }
}

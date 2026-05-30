pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            port: 3000,
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/mydb".to_string()),
        }
    }
}

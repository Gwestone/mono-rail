use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entity::User;
use crate::domain::ports::GetUserPort;

/// PostgreSQL-backed implementation of [`GetUserPort`].
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GetUserPort for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("DB error: {e}"))
    }

    async fn save(&self, user: User) -> Result<User, String> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username, email, password_hash
            "#,
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("DB error: {e}"))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, String> {
        let row: (bool,) = sqlx::query_as(r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#)
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("DB error: {e}"))?;

        Ok(row.0)
    }
}

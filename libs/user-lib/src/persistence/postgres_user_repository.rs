use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entity::User;
use crate::domain::ports::UserRepository;

/// PostgreSQL-backed implementation of the [`UserRepository`] port.
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    /// Creates a new instance of [`PostgresUserRepository`].
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> Result<User, String> {
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
        .map_err(|e| format!("Database error in create: {e}"))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error in find_by_id: {e}"))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Database error in find_by_email: {e}"))
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, String> {
        let row: (bool,) = sqlx::query_as(r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#)
            .bind(email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error in exists_by_email: {e}"))?;

        Ok(row.0)
    }

    async fn update(&self, user: User) -> Result<User, String> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET username = $2, email = $3, password_hash = $4
            WHERE id = $1
            RETURNING id, username, email, password_hash
            "#,
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.email)
        .bind(user.password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Database error in update: {e}"))
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| format!("Database error in delete: {e}"))
    }
}

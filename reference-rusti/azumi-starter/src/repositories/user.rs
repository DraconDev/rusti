use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{CreateUser, User},
    error::Result,
};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_auth_id(&self, auth_id: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE auth_id = $1")
            .bind(auth_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create(&self, user: CreateUser) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (auth_id, email, name, picture)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&user.auth_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.picture)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_name(&self, id: Uuid, name: &str) -> Result<User> {
        let user =
            sqlx::query_as::<_, User>("UPDATE users SET name = $1 WHERE id = $2 RETURNING *")
                .bind(name)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(user)
    }

    pub async fn list_recent(&self, limit: i64) -> Result<Vec<User>> {
        let users =
            sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC LIMIT $1")
                .bind(limit)
                .fetch_all(&self.pool)
                .await?;

        Ok(users)
    }

    pub async fn count_total(&self) -> Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }

    pub async fn count_since(&self, since: chrono::DateTime<chrono::Utc>) -> Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE created_at >= $1")
            .bind(since)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0)
    }
}

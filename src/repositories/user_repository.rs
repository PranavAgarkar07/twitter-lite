use sqlx::PgPool;

use crate::models::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // CREATE USER (must be async)
    pub async fn create(&self, username: String) -> Result<User, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO users (username)
            VALUES ($1)
            RETURNING id, username
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: record.id,
            username: record.username,
        })
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, username
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|row| User {
            id: row.id,
            username: row.username,
        }))
    }
}

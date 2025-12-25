use crate::models::tweet::TweetResponse;
use sqlx::PgPool;

#[derive(Clone)]

pub struct TweetRepository {
    pool: PgPool,
}

impl TweetRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create(&self, content: String) -> Result<TweetResponse, sqlx::Error> {
        let record = sqlx::query!(
            "INSERT INTO tweets (content) VALUES ($1) RETURNING id, content",
            content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(TweetResponse {
            id: record.id as u64,
            content: record.content,
        })
    }
}

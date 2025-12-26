use crate::models::tweet::TweetResponse;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Clone)]
pub struct TweetRepository {
    pool: PgPool,
}

/// Internal DB mapping struct (repository-only)
struct TweetRow {
    id: i32,
    content: String,
    created_at: DateTime<Utc>,
}

impl TweetRepository {
    /// Create a new repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Insert a new tweet
    pub async fn create(&self, content: String) -> Result<TweetResponse, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            INSERT INTO tweets (content)
            VALUES ($1)
            RETURNING id, content
            "#,
            content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(TweetResponse {
            id: record.id as u64,
            content: record.content,
        })
    }

    /// Find a tweet by id
    pub async fn find_by_id(&self, id: i32) -> Result<Option<TweetResponse>, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT id, content
            FROM tweets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.map(|row| TweetResponse {
            id: row.id as u64,
            content: row.content,
        }))
    }

    /// OFFSET-based timeline (kept for learning / comparison)
    pub async fn timeline(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TweetResponse>, sqlx::Error> {
        let records = sqlx::query!(
            r#"
            SELECT id, content
            FROM tweets
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(records
            .into_iter()
            .map(|row| TweetResponse {
                id: row.id as u64,
                content: row.content,
            })
            .collect())
    }

    /// Cursor-based timeline (PRODUCTION-GRADE)
    pub async fn timeline_before(
        &self,
        limit: i64,
        before: Option<(DateTime<Utc>, i32)>,
    ) -> Result<Vec<(TweetResponse, DateTime<Utc>)>, sqlx::Error> {
        let rows: Vec<TweetRow> = match before {
            Some((created_at, id)) => {
                sqlx::query_as!(
                    TweetRow,
                    r#"
                    SELECT
                        id,
                        content,
                        created_at AS "created_at!"
                    FROM tweets

                    WHERE (created_at, id) < ($1, $2)
                    ORDER BY created_at DESC, id DESC
                    LIMIT $3
                    "#,
                    created_at,
                    id,
                    limit
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    TweetRow,
                    r#"
                    SELECT
                        id,
                        content,
                        created_at AS "created_at!"
                    FROM tweets

                    ORDER BY created_at DESC, id DESC
                    LIMIT $1
                    "#,
                    limit
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        Ok(rows
            .into_iter()
            .map(|row| {
                (
                    TweetResponse {
                        id: row.id as u64,
                        content: row.content,
                    },
                    row.created_at,
                )
            })
            .collect())
    }
}

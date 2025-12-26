use sqlx::PgPool;

#[derive(Clone)]
pub struct FollowRepository {
    pool: PgPool,
}

impl FollowRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn follow(&self, follower_id: i32, following_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO follows (follower_id, following_id)
            VALUES ($1, $2)
            "#,
            follower_id,
            following_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn unfollow(&self, follower_id: i32, following_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM follows
            WHERE follower_id = $1 AND following_id = $2
            "#,
            follower_id,
            following_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn is_following(
        &self,
        follower_id: i32,
        following_id: i32,
    ) -> Result<bool, sqlx::Error> {
        let record = sqlx::query!(
            r#"
            SELECT 1 AS exists
            FROM follows
            WHERE follower_id = $1 AND following_id = $2
            "#,
            follower_id,
            following_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(record.is_some())
    }
}

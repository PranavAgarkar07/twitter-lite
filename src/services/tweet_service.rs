use crate::models::tweet::TweetResponse;
use crate::repositories::tweet_repository::TweetRepository;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum TweetServiceError {
    EmptyContent,
    ContentTooLong,
    NotFound,
    DatabaseError,
}

#[derive(Clone)]
pub struct TweetService {
    repository: TweetRepository,
}

impl TweetService {
    pub fn new(repository: TweetRepository) -> Self {
        Self { repository }
    }

    pub async fn create_tweet(&self, content: String) -> Result<TweetResponse, TweetServiceError> {
        if content.trim().is_empty() {
            return Err(TweetServiceError::EmptyContent);
        }

        if content.len() > 280 {
            return Err(TweetServiceError::ContentTooLong);
        }

        self.repository
            .create(content)
            .await
            .map_err(|_| TweetServiceError::DatabaseError)
    }

    pub async fn get_tweet(&self, id: u64) -> Result<TweetResponse, TweetServiceError> {
        self.repository
            .find_by_id(id as i32)
            .await
            .map_err(|_| TweetServiceError::DatabaseError)?
            .ok_or(TweetServiceError::NotFound)
    }

    pub async fn timeline(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TweetResponse>, TweetServiceError> {
        self.repository
            .timeline(limit, offset)
            .await
            .map_err(|_| TweetServiceError::DatabaseError)
    }

    pub async fn timeline_cursor(
        &self,
        limit: i64,
        before: Option<(DateTime<Utc>, i32)>,
    ) -> Result<(Vec<TweetResponse>, Option<String>), TweetServiceError> {
        let rows = self
            .repository
            .timeline_before(limit, before)
            .await
            .map_err(|_| TweetServiceError::DatabaseError)?;

        let next_cursor = rows
            .last()
            .map(|(tweet, ts)| format!("{}|{}", ts.to_rfc3339(), tweet.id));

        let tweets = rows.into_iter().map(|(t, _)| t).collect();

        Ok((tweets, next_cursor))
    }
}

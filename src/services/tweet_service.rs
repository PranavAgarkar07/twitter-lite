use crate::models::tweet::TweetResponse;
use crate::repositories::tweet_repository::TweetRepository;

#[derive(Debug)]
pub enum TweetServiceError {
    EmptyContent,
    ContentTooLong,
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
}

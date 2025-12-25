use axum::{Router, routing::post};

use sqlx::PgPool;

use crate::repositories::tweet_repository::TweetRepository;
use crate::routes::tweets::create_tweet;
use crate::services::tweet_service::TweetService;

#[derive(Clone)]
pub struct AppState {
    pub tweet_service: TweetService,
}

pub fn create_app(pool: PgPool) -> Router {
    let repository = TweetRepository::new(pool);
    let tweet_service = TweetService::new(repository);

    let state = AppState { tweet_service };

    Router::new()
        .route("/tweets", post(create_tweet))
        .with_state(state)
}

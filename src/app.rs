use axum::{
    Router,
    routing::{get, post},
};

use sqlx::PgPool;

use crate::repositories::tweet_repository::TweetRepository;
use crate::routes::tweets::{create_tweet, get_tweet, timeline, timeline_cursor};
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
        .route("/timeline", get(timeline))
        .route("/tweets/:id", get(get_tweet))
        .route("/timeline/cursor", get(timeline_cursor))
        .with_state(state)
}

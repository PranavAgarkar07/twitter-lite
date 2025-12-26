use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;
use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::users::create_user;
use sqlx::PgPool;

use crate::repositories::tweet_repository::TweetRepository;
use crate::routes::tweets::{create_tweet, get_tweet, timeline, timeline_cursor};
use crate::services::tweet_service::TweetService;
//====================
use crate::repositories::follow_repository::FollowRepository;
use crate::routes::follow::{follow, unfollow};
use crate::services::follow_service::FollowService;

#[derive(Clone)]
pub struct AppState {
    pub tweet_service: TweetService,
    pub user_service: UserService,
    pub follow_service: FollowService,
}

pub fn create_app(pool: PgPool) -> Router {
    let user_repository = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repository);
    let tweet_repository = TweetRepository::new(pool.clone());
    let tweet_service = TweetService::new(tweet_repository);
    let follow_repository = FollowRepository::new(pool.clone());
    let follow_service = FollowService::new(follow_repository);

    let state = AppState {
        tweet_service,
        user_service,
        follow_service,
    };
    Router::new()
        .route("/tweets", post(create_tweet))
        .route("/timeline", get(timeline))
        .route("/tweets/:id", get(get_tweet))
        .route("/timeline/cursor", get(timeline_cursor))
        .route("/users", post(create_user))
        .route("/follow", post(follow))
        .route("/follow", axum::routing::delete(unfollow))
        .with_state(state)
}

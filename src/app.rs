use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::routes::tweets::create_tweet;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/tweets", post(create_tweet))
        .with_state(pool)
}

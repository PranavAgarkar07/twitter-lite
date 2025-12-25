use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use sqlx::PgPool;

use crate::models::tweet::{CreateTweetRequest, TweetResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn create_tweet(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTweetRequest>,
) -> Response {
    let rec = sqlx::query!(
        "INSERT INTO tweets (content) VALUES ($1) RETURNING id, content",
        payload.content
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    //temp fake ID (DB comes later)
    let tweet = TweetResponse {
        id: rec.id as u64,
        content: rec.content,
    };

    (StatusCode::CREATED, Json(tweet)).into_response()
}

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{
    app::AppState, models::tweet::CreateTweetRequest, services::tweet_service::TweetServiceError,
};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn create_tweet(
    State(state): State<AppState>,
    Json(payload): Json<CreateTweetRequest>,
) -> Response {
    match state.tweet_service.create_tweet(payload.content).await {
        Ok(tweet) => (StatusCode::CREATED, Json(tweet)).into_response(),

        Err(TweetServiceError::EmptyContent) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Tweet content cannot be empty".into(),
            }),
        )
            .into_response(),

        Err(TweetServiceError::ContentTooLong) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Tweet exceeds 280 characters".into(),
            }),
        )
            .into_response(),

        Err(TweetServiceError::DatabaseError) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".into(),
            }),
        )
            .into_response(),
    }
}

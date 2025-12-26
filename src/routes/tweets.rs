use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState, models::tweet::CreateTweetRequest, services::tweet_service::TweetServiceError,
};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
pub struct TimelineParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CursorTimelineParams {
    limit: Option<i64>,
    before: Option<String>,
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

        Err(TweetServiceError::NotFound) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Unexpected error".into(),
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

pub async fn get_tweet(State(state): State<AppState>, Path(id): Path<u64>) -> Response {
    match state.tweet_service.get_tweet(id).await {
        Ok(tweet) => (StatusCode::OK, Json(tweet)).into_response(),

        Err(TweetServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Tweet not found".into(),
            }),
        )
            .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".into(),
            }),
        )
            .into_response(),
    }
}

pub async fn timeline(
    State(state): State<AppState>,
    Query(params): Query<TimelineParams>,
) -> Response {
    let mut limit = params.limit.unwrap_or(20);
    let mut offset = params.offset.unwrap_or(0);

    // Clamp values (API hardening)
    if limit < 1 {
        limit = 1;
    }
    if limit > 50 {
        limit = 50;
    }
    if offset < 0 {
        offset = 0;
    }

    match state.tweet_service.timeline(limit, offset).await {
        Ok(tweets) => (StatusCode::OK, Json(tweets)).into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".into(),
            }),
        )
            .into_response(),
    }
}

/// Cursor format: "<RFC3339 timestamp>|<tweet_id>"
fn parse_cursor(cursor: &str) -> Option<(DateTime<Utc>, i32)> {
    let (ts, id) = cursor.split_once('|')?;
    let ts = DateTime::parse_from_rfc3339(ts).ok()?.with_timezone(&Utc);
    let id = id.parse::<i32>().ok()?;
    Some((ts, id))
}

pub async fn timeline_cursor(
    State(state): State<AppState>,
    Query(params): Query<CursorTimelineParams>,
) -> Response {
    let mut limit = params.limit.unwrap_or(20);

    // HARD CLAMP (this is mandatory)
    if limit < 1 {
        limit = 1;
    }
    if limit > 50 {
        limit = 50;
    }

    let before = params.before.as_deref().and_then(parse_cursor);

    match state.tweet_service.timeline_cursor(limit, before).await {
        Ok((items, next_cursor)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "items": items,
                "next_cursor": next_cursor
            })),
        )
            .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".into(),
            }),
        )
            .into_response(),
    }
}

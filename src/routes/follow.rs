use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};

use crate::{app::AppState, services::follow_service::FollowServiceError};

#[derive(Deserialize)]
pub struct FollowRequest {
    pub follower_id: i32,
    pub following_id: i32,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn follow(State(state): State<AppState>, Json(payload): Json<FollowRequest>) -> Response {
    match state
        .follow_service
        .follow(payload.follower_id, payload.following_id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),

        Err(FollowServiceError::CannotFollowSelf) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Cannot follow yourself".into(),
            }),
        )
            .into_response(),

        Err(FollowServiceError::AlreadyFollowing) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Already following this user".into(),
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

pub async fn unfollow(
    State(state): State<AppState>,
    Json(payload): Json<FollowRequest>,
) -> Response {
    match state
        .follow_service
        .unfollow(payload.follower_id, payload.following_id)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),

        Err(FollowServiceError::NotFollowing) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "You are not following this user".into(),
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

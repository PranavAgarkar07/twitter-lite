use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::{
    app::AppState, models::user::CreateUserRequest, services::user_service::UserServiceError,
};

use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Response {
    match state.user_service.create_user(payload.username).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),

        Err(UserServiceError::EmptyUsername) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Username cannot be empty".into(),
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

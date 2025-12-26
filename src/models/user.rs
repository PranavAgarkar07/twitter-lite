use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

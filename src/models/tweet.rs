use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct TweetResponse {
    pub id: u64,
    pub content: String,
}

use crate::{models::user::User, repositories::user_repository::UserRepository};

#[derive(Debug)]
pub enum UserServiceError {
    EmptyUsername,
    DatabaseError,
}

#[derive(Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, username: String) -> Result<User, UserServiceError> {
        if username.trim().is_empty() {
            return Err(UserServiceError::EmptyUsername);
        }

        self.repository
            .create(username)
            .await
            .map_err(|_| UserServiceError::DatabaseError)
    }
}

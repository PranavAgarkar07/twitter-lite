use crate::repositories::follow_repository::FollowRepository;

#[derive(Debug)]
pub enum FollowServiceError {
    CannotFollowSelf,
    AlreadyFollowing,
    NotFollowing,
    DatabaseError,
}

#[derive(Clone)]
pub struct FollowService {
    repository: FollowRepository,
}

impl FollowService {
    pub fn new(repository: FollowRepository) -> Self {
        Self { repository }
    }

    pub async fn follow(
        &self,
        follower_id: i32,
        following_id: i32,
    ) -> Result<(), FollowServiceError> {
        // Rule 1: cannot follow yourself
        if follower_id == following_id {
            return Err(FollowServiceError::CannotFollowSelf);
        }

        // Rule 2: prevent duplicate follow
        let already_following = self
            .repository
            .is_following(follower_id, following_id)
            .await
            .map_err(|_| FollowServiceError::DatabaseError)?;

        if already_following {
            return Err(FollowServiceError::AlreadyFollowing);
        }

        // Perform follow
        self.repository
            .follow(follower_id, following_id)
            .await
            .map_err(|_| FollowServiceError::DatabaseError)?;

        Ok(())
    }

    pub async fn unfollow(
        &self,
        follower_id: i32,
        following_id: i32,
    ) -> Result<(), FollowServiceError> {
        // Rule: unfollow is idempotent
        let is_following = self
            .repository
            .is_following(follower_id, following_id)
            .await
            .map_err(|_| FollowServiceError::DatabaseError)?;

        if !is_following {
            return Err(FollowServiceError::NotFollowing);
        }

        self.repository
            .unfollow(follower_id, following_id)
            .await
            .map_err(|_| FollowServiceError::DatabaseError)?;

        Ok(())
    }
}

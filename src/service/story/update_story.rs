use crate::{domain::Story, repo::StoryRepo, service::UseCase, Result};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Updates stories
pub struct UpdateStory {
    pub repo: Arc<StoryRepo>,
}

#[async_trait]
impl UseCase for UpdateStory {
    /// Input is a story id and updated name
    type Req = (i32, String);

    /// Output is the updated story
    type Rep = Result<Story>;

    /// Update a story if it exists.
    async fn execute(&self, (id, name): Self::Req) -> Self::Rep {
        self.repo
            .fetch(id)
            .and_then(|_| self.repo.update(id, name))
            .await
    }
}
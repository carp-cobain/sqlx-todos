use crate::{
    domain::Task,
    repo::{StoryRepo, TaskRepo},
    service::UseCase,
    Result,
};
use async_trait::async_trait;
use futures_util::TryFutureExt;
use std::sync::Arc;

/// Get story tasks.
pub struct GetTasks {
    pub story_repo: Arc<StoryRepo>,
    pub task_repo: Arc<TaskRepo>,
}

#[async_trait]
impl UseCase for GetTasks {
    /// Input is a story id
    type Req = i32;

    /// Output is a vector of tasks
    type Rep = Result<Vec<Task>>;

    /// Get all tasks for a story if it exists.
    async fn execute(&self, story_id: Self::Req) -> Self::Rep {
        tracing::debug!("execute: story_id={}", story_id);
        self.story_repo
            .fetch(story_id)
            .and_then(|_| self.task_repo.list(story_id))
            .await
    }
}

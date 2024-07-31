use crate::{domain::Task, repo::Repo, Result};
use futures_util::{Future, TryFutureExt};
use std::{
    ops::{AsyncFnOnce, Deref},
    pin::Pin,
    sync::Arc,
};

/// Create a new task.
pub struct CreateTask(pub Arc<Repo>);

// Allows direct calls to inner repo.
impl Deref for CreateTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Call as an async function.
impl AsyncFnOnce<(i32, String)> for CreateTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: (i32, String)) -> Self::CallOnceFuture {
        let (story_id, name) = args;
        Box::pin(async move {
            self.fetch_story(story_id)
                .and_then(|_| self.create_task(story_id, name))
                .await
        })
    }
}

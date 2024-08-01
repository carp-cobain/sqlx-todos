use crate::{domain::Task, repo::Repo, Result};
use futures_util::{Future, TryFutureExt};
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const create_task: CreateTask = CreateTask;

/// Creates new tasks.
pub struct CreateTask;

/// Sugar for function inputs.m
type Args = (Arc<Repo>, i32, String);

/// Call as an async function.
impl AsyncFnOnce<Args> for CreateTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (repo, story_id, name) = args;
        Box::pin(async move {
            repo.fetch_story(story_id)
                .and_then(|_| repo.create_task(story_id, name))
                .await
        })
    }
}

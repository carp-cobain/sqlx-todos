use crate::{domain::Task, repo::Repo, Result};
use futures_util::Future;
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const get_tasks: GetTasks = GetTasks;

/// Get story tasks.
pub struct GetTasks;

/// Sugar for function inputs.
type Args = (Arc<Repo>, i32);

/// Call as an async function.
impl AsyncFnOnce<Args> for GetTasks {
    type Output = Result<Vec<Task>>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (repo, story_id) = args;
        Box::pin(async move {
            // Try and query for tasks first.
            let tasks = repo.list_tasks(story_id).await?;

            // When zero tasks were returned, check whether the story exists.
            // This is an optimization; if tasks were returned, the story DOES exist
            // and no further querying is required.
            if tasks.is_empty() {
                // Only care about errors here
                let _ = repo.fetch_story(story_id).await?;
            }

            Ok(tasks)
        })
    }
}

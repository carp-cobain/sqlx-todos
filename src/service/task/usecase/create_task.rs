use crate::{
    domain::{Status, Task},
    repo::Repo,
    Result,
};
use futures_util::{Future, TryFutureExt};
use std::{
    ops::{AsyncFnOnce, Deref},
    pin::Pin,
    sync::Arc,
};

/// Use case for creating new tasks.
pub struct CreateTask(pub Arc<Repo>);

// Use case function outputs.
type Res = Result<Task>;
type ResFut = Pin<Box<dyn Future<Output = Res> + Send>>;

/// Call as an async function with 2 args.
impl AsyncFnOnce<(i32, String)> for CreateTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, args: (i32, String)) -> Self::CallOnceFuture {
        let (story_id, name) = args;
        Box::pin(async move {
            self.fetch_story(story_id)
                .and_then(|_| self.create_task(story_id, name, Default::default()))
                .await
        })
    }
}

/// Arguments: story_id, name, and status.
type Args = (i32, String, Status);

/// Call as an async function with 3 args.
impl AsyncFnOnce<Args> for CreateTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (story_id, name, status) = args;
        Box::pin(async move {
            self.fetch_story(story_id)
                .and_then(|_| self.create_task(story_id, name, status))
                .await
        })
    }
}

// Allow calls directly to the inner repo.
impl Deref for CreateTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

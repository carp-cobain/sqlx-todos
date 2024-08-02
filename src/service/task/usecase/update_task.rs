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

/// Updates tasks
pub struct UpdateTask(pub Arc<Repo>);

// Function inputs
type Args = (i32, String, Status);
type ArgsNm = (i32, String);
type ArgsSt = (i32, Status);

// Function outputs
type Res = Result<Task>;
type ResFut = Pin<Box<dyn Future<Output = Res> + Send>>;

// Call as an async function with id, name, and status args.
impl AsyncFnOnce<Args> for UpdateTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (id, name, status) = args;
        tracing::debug!("args: id={}, name={}, status={}", id, name, status);

        Box::pin(async move {
            // Fetch the task and update its name and status.
            self.fetch_task(id)
                .and_then(|_| self.update_task(id, name, status))
                .await
        })
    }
}

// Call as an async function with id and name args.
impl AsyncFnOnce<ArgsNm> for UpdateTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, args: ArgsNm) -> Self::CallOnceFuture {
        let (id, name) = args;
        tracing::debug!("args: id={}. name={}", id, name);

        Box::pin(async move {
            // Fetch the task and update its name.
            self.fetch_task(id)
                .and_then(|t| self.update_task(id, name, t.status))
                .await
        })
    }
}

// Call as an async function with id and status args.
impl AsyncFnOnce<ArgsSt> for UpdateTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, args: ArgsSt) -> Self::CallOnceFuture {
        let (id, status) = args;
        tracing::debug!("args: id={}. status={}", id, status);

        Box::pin(async move {
            // Fetch the task and update its status.
            self.fetch_task(id)
                .and_then(|t| self.update_task(id, t.name, status))
                .await
        })
    }
}

// Allow calls directly to the inner repo.
impl Deref for UpdateTask {
    type Target = Arc<Repo>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

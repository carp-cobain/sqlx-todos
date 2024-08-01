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

/// Call as an async function with id, name, and status args.
impl AsyncFnOnce<(i32, String, Status)> for UpdateTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(
        self,
        args: (i32, String, Status),
    ) -> Self::CallOnceFuture {
        let (id, name, status) = args;
        tracing::debug!("args: id={}, name={}, status={}", id, name, status);

        Box::pin(async move {
            // Fetch the task and update it's name and status.
            self.fetch_task(id)
                .and_then(|_| self.update_task(id, name, status))
                .await
        })
    }
}

/// Call as an async function with id and name args.
impl AsyncFnOnce<(i32, String)> for UpdateTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: (i32, String)) -> Self::CallOnceFuture {
        let (id, name) = args;
        tracing::debug!("args: id={}. name={}", id, name);

        Box::pin(async move {
            // Fetch the task and update it's name.
            self.fetch_task(id)
                .and_then(|t| self.update_task(id, name, t.status))
                .await
        })
    }
}

/// Call as an async function with id and status args.
impl AsyncFnOnce<(i32, Status)> for UpdateTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: (i32, Status)) -> Self::CallOnceFuture {
        let (id, status) = args;
        tracing::debug!("args: id={}. status={}", id, status);

        Box::pin(async move {
            // Fetch the task and update it's status.
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

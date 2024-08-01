use crate::{
    domain::{Status, Task},
    repo::Repo,
    Error, Result,
};
use futures_util::{Future, TryFutureExt};
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const update_task: UpdateTask = UpdateTask;

/// Update tasks.
pub struct UpdateTask;

type Args = (Arc<Repo>, i32, Option<String>, Option<Status>);

/// Call as an async function.
impl AsyncFnOnce<Args> for UpdateTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (repo, id, name_opt, status_opt) = args;
        Box::pin(async move {
            // Make sure an update was provided.
            if name_opt.is_none() && status_opt.is_none() {
                let error = Error::invalid_args("no task updates provided");
                return Err(error);
            }
            // Fetch the task and update it.
            repo.fetch_task(id)
                .and_then(|task| {
                    let name = name_opt.unwrap_or(task.name);
                    let status = status_opt.unwrap_or(task.status);
                    repo.update_task(id, name, status)
                })
                .await
        })
    }
}

use crate::{domain::Task, repo::Repo, Result};
use futures_util::Future;
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const get_task: GetTask = GetTask;

/// Gets tasks by id.
pub struct GetTask;

/// Sugar for function inputs.
type Args = (Arc<Repo>, i32);

/// Call as an async function.
impl AsyncFnOnce<Args> for GetTask {
    type Output = Result<Task>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (repo, id) = args;
        Box::pin(async move { repo.fetch_task(id).await })
    }
}

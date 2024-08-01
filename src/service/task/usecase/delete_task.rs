use crate::{repo::Repo, Result};
use futures_util::{Future, TryFutureExt};
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const delete_task: DeleteTask = DeleteTask;

/// Deletes tasks.
pub struct DeleteTask;

/// Sugar for function inputs.
type Args = (Arc<Repo>, i32);

/// Call as an async function.
impl AsyncFnOnce<Args> for DeleteTask {
    type Output = Result<()>;
    type CallOnceFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    extern "rust-call" fn async_call_once(self, args: Args) -> Self::CallOnceFuture {
        let (repo, id) = args;
        Box::pin(async move {
            repo.fetch_task(id)
                .and_then(|_| repo.delete_task(id))
                .await
                .map(|_| ())
        })
    }
}

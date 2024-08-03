use crate::{repo::Repo, Result};
use futures_util::{Future, TryFutureExt};
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const delete_task: DeleteTask = DeleteTask;

/// Deletes tasks.
pub struct DeleteTask;

// Function inputs.
type Args = (Arc<Repo>, i32);

// Function outputs.
type Res = Result<()>;
type ResFut = Pin<Box<dyn Future<Output = Res> + Send>>;

// Call as an async function.
impl AsyncFnOnce<Args> for DeleteTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, (repo, id): Args) -> Self::CallOnceFuture {
        Box::pin(async move {
            repo.fetch_task(id)
                .and_then(|_| repo.delete_task(id))
                .await
                .map(|_| ())
        })
    }
}

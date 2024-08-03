use crate::{domain::Task, repo::Repo, Result};
use futures_util::Future;
use std::{ops::AsyncFnOnce, pin::Pin, sync::Arc};

#[allow(non_upper_case_globals)]
pub const get_task: GetTask = GetTask;

/// Gets tasks by id.
pub struct GetTask;

// Function inputs.
type Args = (Arc<Repo>, i32);

// Function outputs.
type Res = Result<Task>;
type ResFut = Pin<Box<dyn Future<Output = Res> + Send>>;

// Call as an async function.
impl AsyncFnOnce<Args> for GetTask {
    type Output = Res;
    type CallOnceFuture = ResFut;

    extern "rust-call" fn async_call_once(self, (repo, id): Args) -> Self::CallOnceFuture {
        Box::pin(async move { repo.fetch_task(id).await })
    }
}

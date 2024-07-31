use async_trait::async_trait;

/// Services for stories.
pub mod story;

/// Services for tasks.
pub mod task;

/// Defines a single unit of business logic.
/// Inspired by Finagle's Service type: `trait Service[Req, Rep] extends (Req => Future[Rep])`
#[async_trait]
pub trait UseCase: Sized + Send + Sync + 'static {
    /// Input arguments
    type Req: Send + 'static;

    /// Output results
    type Rep: Send + 'static;

    /// Business logic
    async fn execute(&self, req: Self::Req) -> Self::Rep;
}

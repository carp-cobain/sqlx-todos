use axum::Router;
use std::sync::Arc;

mod ctx;
mod dto;
mod page;
mod story;
mod task;

pub use ctx::Ctx;

/// The top-level API
pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    /// Create a new api
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    /// Define API routes, mapping paths to handlers.
    pub fn routes(self) -> Router {
        story::routes().merge(task::routes()).with_state(self.ctx)
    }
}

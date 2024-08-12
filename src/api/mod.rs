use axum::Router;
use std::sync::Arc;
use utoipa::{openapi::OpenApi as OpenApiDocs, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

mod ctx;
mod dto;
pub mod routes;
mod tracer;

pub use ctx::Ctx;

use routes::{status, story, task};

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
        tracer::wrap(
            Router::new()
                .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs()))
                .merge(status::routes())
                .merge(story::routes())
                .merge(task::routes()),
        )
        .with_state(self.ctx)
    }
}

/// Combine OpenApi docs for internal routes.
pub fn docs() -> OpenApiDocs {
    let mut api = story::ApiDoc::openapi();
    api.merge(task::ApiDoc::openapi());
    api
}

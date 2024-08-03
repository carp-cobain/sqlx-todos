use crate::{
    api::dto::story::StoryBody,
    api::page::{Page, PageParams, PageToken},
    api::Ctx,
    Result,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;

/// API routes for stories
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id", get(get_story).delete(delete_story).patch(update_story))
        .route("/stories/:id/tasks", get(get_tasks))
}

/// Get story by id
async fn get_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}", id);
    let story = ctx.stories.fetch_story(id).await?;
    Ok(Json(story))
}

/// Get a page of stories
async fn get_stories(
    params: Option<Query<PageParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories");

    let q = params.unwrap_or_default();
    let page_id = PageToken::decode_or(&q.page_token, i32::MAX)?;

    let (next_page, stories) = ctx.stories.list(page_id, q.page_size()).await?;
    let page = Page::new(PageToken::encode(next_page), stories);

    Ok(Json(page))
}

/// Get all tasks for a story
async fn get_tasks(
    Path(story_id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}/tasks", story_id);
    let tasks = ctx.tasks.list(story_id).await?;
    Ok(Json(tasks))
}

/// Create a new story for an owner
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /stories");
    let name = body.validate()?;
    let story = ctx.stories.create_story(name).await?;
    Ok((StatusCode::CREATED, Json(story)))
}

/// Update a story name.
async fn update_story(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("PATCH /stories/{}", id);
    let name = body.validate()?;
    let story = ctx.stories.update(id, name).await?;
    Ok(Json(story))
}

/// Delete a story by id
async fn delete_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /stories/{}", id);
    if let Err(err) = ctx.stories.delete(id).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}

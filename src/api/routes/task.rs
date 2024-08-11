use crate::{
    api::dto::task::{CreateTaskBody, PatchTaskBody},
    api::Ctx,
    domain::{Status, Task},
    error::ErrorDto,
    Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use uuid::Uuid;

/// OpenApi docs for story routes
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(get_task, create_task, update_task, delete_task),
    components(schemas(Task, Status, CreateTaskBody, PatchTaskBody, ErrorDto)),
    tags((name = "Task"))
)]
pub struct ApiDoc;

/// API routes for tasks
#[rustfmt::skip]
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_task).delete(delete_task).patch(update_task))
}

/// Get a task
#[utoipa::path(
    get,
    tag = "Task",
    path = "/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "Task id")
    ),
    responses(
        (status = 200, description = "Get a task by id", body = Task),
        (status = 404, description = "Task not found", body = ErrorDto)
    )
)]
async fn get_task(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Task>> {
    let task = ctx.fetch_task(id).await?;
    Ok(Json(task))
}

/// Create a task
#[utoipa::path(
    post,
    tag = "Task",
    path = "/tasks",
    request_body = CreateTaskBody,
    responses(
        (status = 201, description = "Task created", body = Task),
        (status = 400, description = "Invalid requesst body", body = ErrorDto)
    )
)]
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<CreateTaskBody>,
) -> Result<impl IntoResponse> {
    let (story_id, name, status) = body.validate()?;
    let task = ctx
        .fetch_story(story_id)
        .and_then(|_| ctx.create_task(story_id, name, status))
        .await?;
    Ok((StatusCode::CREATED, Json(task)))
}

/// Update a task
#[utoipa::path(
    patch,
    tag = "Task",
    path = "/tasks/{id}",
    request_body = PatchTaskBody,
    responses(
        (status = 200, description = "Task updated", body = Task),
        (status = 400, description = "Invalid request body", body = ErrorDto),
        (status = 404, description = "Task not found", body = ErrorDto)
    )
)]
async fn update_task(
    Path(id): Path<Uuid>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<PatchTaskBody>,
) -> Result<Json<Task>> {
    let (name_opt, status_opt) = body.validate()?;
    let task = ctx
        .fetch_task(id)
        .and_then(|task| {
            let name = name_opt.unwrap_or(task.name);
            let status = status_opt.unwrap_or(task.status);
            ctx.update_task(id, name, status)
        })
        .await?;
    Ok(Json(task))
}

/// Delete a task
#[utoipa::path(
    delete,
    tag = "Task",
    path = "/tasks/{id}",
    params(
        ("id" = Uuid, Path, description = "The task id")
    ),
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found")
    )
)]
async fn delete_task(Path(id): Path<Uuid>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    if let Err(err) = ctx.fetch_task(id).and_then(|_| ctx.delete_task(id)).await {
        return StatusCode::from(err);
    }
    StatusCode::NO_CONTENT
}

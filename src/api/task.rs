use crate::{
    api::dto::{CreateTaskBody, PatchTaskBody},
    api::Ctx,
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

/// API routes for tasks
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new().route("/tasks", post(create_task)).route(
        "/tasks/:id",
        get(get_task).delete(delete_task).patch(update_task),
    )
}

/// Get a task by id
async fn get_task(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /tasks/{}", id);
    let task = ctx.repo.select_task(id).await?;
    Ok(Json(task))
}

/// Create a new task
async fn create_task(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<CreateTaskBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /tasks");

    let (story_id, name) = body.validate()?;
    let task = ctx
        .repo
        .select_story(story_id)
        .and_then(|story| ctx.repo.insert_task(story.id, name))
        .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

/// Delete a task by id
async fn delete_task(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /tasks/{}", id);

    let result = ctx
        .repo
        .select_task(id)
        .and_then(|_| ctx.repo.delete_task(id))
        .await;

    match result {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(err) => StatusCode::from(err),
    }
}

/// Update a task.
async fn update_task(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<PatchTaskBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("PATCH /tasks/{}", id);

    // Validate
    let (name, status) = body.validate()?;
    let existing = ctx.repo.select_task(id).await?;

    // Unwrap updated fields falling back to existing values.
    let name = name.unwrap_or(existing.name);
    let status = status.unwrap_or(existing.status);

    // Update
    let task = ctx.repo.update_task(id, name, status).await?;
    Ok(Json(task))
}

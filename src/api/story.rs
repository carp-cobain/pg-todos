use crate::{
    api::dto::StoryBody,
    api::page::{Page, PageParams, PageToken},
    api::Ctx,
    domain::Story,
    Result,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;

/// API routes for stories
pub fn routes() -> Router<Arc<Ctx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id/tasks", get(get_tasks))
        .route(
            "/stories/:id",
            get(get_story).delete(delete_story).patch(update_story),
        )
}

/// Get story by id
async fn get_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<Json<Story>> {
    tracing::info!("GET /stories/{}", id);
    let story = ctx.repo.select_story(id).await?;
    Ok(Json(story))
}

/// Get a page of the most recently created stories.
async fn get_stories(
    params: Option<Query<PageParams>>,
    State(ctx): State<Arc<Ctx>>,
) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories");

    let q = params.unwrap_or_default();
    let page_id = PageToken::decode_or(&q.page_token, std::i32::MAX)?;

    let stories = ctx.repo.select_stories(page_id).await?;
    let next_page = stories.last().map(|s| s.id - 1).unwrap_or_default();
    let page = Page::new(PageToken::encode(next_page), stories);

    Ok(Json(page))
}

/// Get tasks for a story
async fn get_tasks(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> Result<impl IntoResponse> {
    tracing::info!("GET /stories/{}/tasks", id);
    let tasks = ctx.repo.select_tasks(id).await?;
    Ok(Json(tasks))
}

/// Create a new story
async fn create_story(
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("POST /stories");

    let name = body.validate()?;
    let story = ctx.repo.insert_story(name).await?;

    Ok((StatusCode::CREATED, Json(story)))
}

/// Delete a story by id
async fn delete_story(Path(id): Path<i32>, State(ctx): State<Arc<Ctx>>) -> StatusCode {
    tracing::info!("DELETE /stories/{}", id);

    let result = ctx
        .repo
        .select_story(id)
        .and_then(|_| ctx.repo.delete_story(id))
        .await;

    match result {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(err) => StatusCode::from(err),
    }
}

/// Update an existing story name
async fn update_story(
    Path(id): Path<i32>,
    State(ctx): State<Arc<Ctx>>,
    Json(body): Json<StoryBody>,
) -> Result<impl IntoResponse> {
    tracing::info!("PATCH /stories/{}", id);

    let name = body.validate()?;
    let story = ctx
        .repo
        .select_story(id)
        .and_then(|_| ctx.repo.update_story(id, name))
        .await?;

    Ok(Json(story))
}

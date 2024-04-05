use crate::{
    api::{dto::CreateStoryBody, ApiCtx},
    domain::Story,
    Result,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures_util::TryFutureExt;
use std::sync::Arc;
use validator::Validate;

/// API routes for stories
pub fn routes() -> Router<Arc<ApiCtx>> {
    Router::new()
        .route("/stories", get(get_stories).post(create_story))
        .route("/stories/:id", get(get_story).delete(delete_story))
}

/// Get story by id
async fn get_story(Path(id): Path<i32>, State(ctx): State<Arc<ApiCtx>>) -> Result<Json<Story>> {
    tracing::debug!("get_story: {}", id);
    let story = ctx.repo.select_story(id).await?;
    Ok(Json(story))
}

/// Get stories by owner
async fn get_stories(State(ctx): State<Arc<ApiCtx>>) -> Result<Json<Vec<Story>>> {
    tracing::debug!("get_stories");
    let stories = ctx.repo.select_stories().await?;
    Ok(Json(stories))
}

/// Create a new story for an owner
async fn create_story(
    State(ctx): State<Arc<ApiCtx>>,
    Json(body): Json<CreateStoryBody>,
) -> Result<impl IntoResponse> {
    tracing::debug!("create_story: {:?}", body);

    body.validate()?;
    let story = ctx.repo.insert_story(body.name).await?;

    Ok((StatusCode::CREATED, Json(story)))
}

/// Delete a story by id
async fn delete_story(Path(id): Path<i32>, State(ctx): State<Arc<ApiCtx>>) -> StatusCode {
    tracing::debug!("delete_story: {}", id);

    let result = ctx
        .repo
        .select_story(id)
        .and_then(|_| ctx.repo.delete_story(id))
        .await;

    match result {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(error) => StatusCode::from(error),
    }
}

use super::Ctx;
use axum::{routing::get, Router};
use std::sync::Arc;

/// API route for health checks
pub fn routes() -> Router<Arc<Ctx>> {
    let handler = || async { "øk" };
    Router::new()
        .route("/status", get(handler))
        .route("/status/*glob", get(handler))
}

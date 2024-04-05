use axum::Router;
use std::sync::Arc;

mod ctx;
mod dto;
mod story;

pub use ctx::ApiCtx;

pub struct Api {
    ctx: Arc<ApiCtx>,
}

impl Api {
    pub fn new(ctx: Arc<ApiCtx>) -> Self {
        Self { ctx }
    }

    pub fn routes(self) -> Router {
        story::routes().with_state(self.ctx)
    }
}

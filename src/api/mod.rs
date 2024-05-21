use axum::Router;
use std::sync::Arc;

mod ctx;
mod dto;
mod status;
mod story;

pub use ctx::Ctx;

pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    pub fn routes(self) -> Router {
        status::routes().merge(story::routes()).with_state(self.ctx)
    }
}

use axum::Router;
use std::sync::Arc;

mod ctx;
mod dto;
mod page;
mod status;
mod story;
mod task;

pub use ctx::Ctx;

pub struct Api {
    ctx: Arc<Ctx>,
}

impl Api {
    pub fn new(ctx: Arc<Ctx>) -> Self {
        Self { ctx }
    }

    pub fn routes(self) -> Router {
        status::routes()
            .merge(story::routes())
            .merge(task::routes())
            .with_state(self.ctx)
    }
}

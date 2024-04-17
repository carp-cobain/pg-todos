use crate::{config::Config, repo::Repo};
use axum::Router;
use std::sync::Arc;

mod status;
mod story;

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

#[derive(Clone)]
pub struct Ctx {
    pub config: Arc<Config>,
    pub repo: Arc<Repo>,
}

impl Ctx {
    pub async fn new(config: Arc<Config>) -> Self {
        let config = Arc::clone(&config);
        let repo = Arc::new(Repo::new(&config.db_url).await);
        Self { config, repo }
    }
}

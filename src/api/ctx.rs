use crate::{config::Config, repo::Repo};
use std::sync::Arc;

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

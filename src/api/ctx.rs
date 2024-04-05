use crate::{config::Config, repo::Repo};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiCtx {
    pub config: Arc<Config>,
    pub repo: Arc<Repo>,
}

impl ApiCtx {
    pub async fn new(config: Arc<Config>) -> Self {
        let config = Arc::clone(&config);
        let repo = Repo::new(&config.db_connection_string()).await;
        Self {
            config,
            repo: Arc::new(repo),
        }
    }
}

use super::sql;
use tokio_postgres::{Client, Statement};

/// Pre-cached prepared statements
pub(crate) struct Cache {
    // Story
    pub select_story: Statement,
    pub select_stories: Statement,
    pub insert_story: Statement,
    pub update_story: Statement,
    pub delete_story: Statement,
    // Tasks
    pub select_task: Statement,
    pub select_tasks: Statement,
    pub insert_task: Statement,
    pub update_task: Statement,
    pub delete_task: Statement,
}

impl Cache {
    /// Prepare and cache SQL statements for a tokio_postgres client.
    /// NOTE: Statements must be executed by the client passed in here.
    pub(crate) async fn prepare(client: &Client) -> Self {
        Self {
            // Story
            select_story: client.prepare(sql::stories::FETCH).await.unwrap(),
            select_stories: client.prepare(sql::stories::SELECT).await.unwrap(),
            insert_story: client.prepare(sql::stories::INSERT).await.unwrap(),
            update_story: client.prepare(sql::stories::UPDATE).await.unwrap(),
            delete_story: client.prepare(sql::stories::DELETE).await.unwrap(),
            // Task
            select_task: client.prepare(sql::tasks::FETCH).await.unwrap(),
            select_tasks: client.prepare(sql::tasks::SELECT).await.unwrap(),
            insert_task: client.prepare(sql::tasks::INSERT).await.unwrap(),
            update_task: client.prepare(sql::tasks::UPDATE).await.unwrap(),
            delete_task: client.prepare(sql::tasks::DELETE).await.unwrap(),
        }
    }
}

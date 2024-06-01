use super::Repo;
use crate::{domain::Story, Error, Result};
use futures::StreamExt;
use tokio::pin;

impl Repo {
    /// Select a story by id
    pub async fn select_story(&self, id: i32) -> Result<Story> {
        tracing::debug!("select_story: {}", id);

        let stream = self
            .client
            .query_raw(&self.statements.select_story, &[&id])
            .await?;
        pin!(stream);

        if let Some(result) = stream.next().await {
            let row = result?;
            Ok(Story::new(row.get(0), row.get(1)))
        } else {
            Err(Error::not_found(format!("story not found: {}", id)))
        }
    }

    /// Select a page of stories
    pub async fn select_stories(&self, page_id: i32) -> Result<Vec<Story>> {
        tracing::debug!("select_stories: page_id={}", page_id);

        let stream = self
            .client
            .query_raw(&self.statements.select_stories, &[page_id])
            .await?;
        pin!(stream);

        let mut stories = Vec::with_capacity(100);
        while let Some(result) = stream.next().await {
            let row = result?;
            stories.push(Story::new(row.get(0), row.get(1)));
        }

        Ok(stories)
    }

    /// Insert a new story
    pub async fn insert_story(&self, name: String) -> Result<Story> {
        tracing::debug!("insert_story: {}", name);

        let stream = self
            .client
            .query_raw(&self.statements.insert_story, &[&name])
            .await?;
        pin!(stream);

        if let Some(result) = stream.next().await {
            let row = result?;
            Ok(Story::new(row.get(0), name))
        } else {
            Err(Error::internal(format!("failed to insert story: {}", name)))
        }
    }

    /// Delete a story.
    pub async fn delete_story(&self, id: i32) -> Result<()> {
        tracing::debug!("delete_story: {}", id);

        self.client
            .execute(&self.statements.delete_story, &[&id])
            .await?;

        Ok(())
    }

    /// Update story name.
    pub async fn update_story(&self, id: i32, name: String) -> Result<Story> {
        tracing::debug!("update_story: {}, {}", id, name);

        self.client
            .execute(&self.statements.update_story, &[&name, &id])
            .await?;

        Ok(Story::new(id, name))
    }
}

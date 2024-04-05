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
            Ok(Story {
                id: row.get(0),
                name: row.get(1),
            })
        } else {
            Err(Error::NotFound {
                message: format!("story not found: {}", id),
            })
        }
    }

    /// Select a page of stories
    pub async fn select_stories(&self) -> Result<Vec<Story>> {
        tracing::debug!("select_stories");

        let stream = self
            .client
            .query_raw::<_, _, &[i32; 0]>(&self.statements.select_stories, &[])
            .await?;

        pin!(stream);

        let mut stories = Vec::with_capacity(32);

        while let Some(result) = stream.next().await {
            let row = result?;
            stories.push(Story {
                id: row.get(0),
                name: row.get(1),
            });
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
            Ok(Story {
                id: row.get(0),
                name,
            })
        } else {
            Err(Error::Internal {
                message: format!("failed to insert story: {}", name),
            })
        }
    }

    /// Delete a story.
    pub async fn delete_story(&self, id: i32) -> Result<u64> {
        tracing::debug!("delete_story: {}", id);
        self.client
            .execute_raw(&self.statements.delete_story, &[&id])
            .await
            .map_err(Error::from)
    }
}

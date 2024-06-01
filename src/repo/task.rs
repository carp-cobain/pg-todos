use crate::{domain::Task, repo::Repo, Error, Result};
use tokio_postgres::Row;

/// Row mapper for the task domain object.
impl From<&Row> for Task {
    fn from(row: &Row) -> Self {
        Task::new(row.get(0), row.get(1), row.get(2), row.get(3))
    }
}

impl Repo {
    /// Select a task by id
    pub async fn select_task(&self, id: i32) -> Result<Task> {
        tracing::debug!("select_task: {}", id);

        let result = self
            .client
            .query_one(&self.statements.select_task, &[&id])
            .await;

        if let Ok(row) = result {
            Ok(Task::from(&row))
        } else {
            Err(Error::not_found(format!("task not found: {}", id)))
        }
    }

    /// Select a page of tasks for a story.
    pub async fn select_tasks(&self, story_id: i32) -> Result<Vec<Task>> {
        tracing::debug!("select_tasks: {}", story_id);

        let tasks: Vec<_> = self
            .client
            .query(&self.statements.select_tasks, &[&story_id])
            .await?
            .iter()
            .map(Task::from)
            .collect();

        Ok(tasks)
    }

    /// Insert a new task
    pub async fn insert_task(&self, story_id: i32, name: String) -> Result<Task> {
        tracing::debug!("insert_task: {}, {}", story_id, name);

        let row = self
            .client
            .query_one(&self.statements.insert_task, &[&story_id, &name])
            .await?;

        Ok(Task::new(row.get(0), story_id, name, row.get(1)))
    }

    /// Delete a task.
    pub async fn delete_task(&self, id: i32) -> Result<()> {
        tracing::debug!("delete_task: {}", id);

        self.client
            .execute(&self.statements.delete_task, &[&id])
            .await?;

        Ok(())
    }

    /// Update task name and status.
    pub async fn update_task(&self, id: i32, name: String, status: String) -> Result<Task> {
        tracing::debug!("update_task: {}, {}, {}", id, name, status);

        let row = self
            .client
            .query_one(&self.statements.update_task, &[&name, &status, &id])
            .await?;

        Ok(Task::new(id, row.get(0), name, status))
    }
}

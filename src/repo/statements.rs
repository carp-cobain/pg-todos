use tokio_postgres::{Client, Statement};

// SQL queries for the stories table
const SQL_SELECT_STORY: &str = "select id, name from stories where id = $1";
const SQL_SELECT_STORIES: &str = "select id, name from stories order by id limit 10";
const SQL_INSERT_STORY: &str = "insert into stories (name) values ($1) returning id";
const SQL_DELETE_STORY: &str = "delete from stories where id = $1";

/// Grouped prepared statements
pub(crate) struct Statements {
    pub select_story: Statement,
    pub select_stories: Statement,
    pub insert_story: Statement,
    pub delete_story: Statement,
}

impl Statements {
    /// Prepare SQL statements for a pg client.
    /// NOTE: Statements must be executed by the creating client.
    pub(crate) async fn prepare(client: &Client) -> Self {
        Self {
            select_story: client.prepare(SQL_SELECT_STORY).await.unwrap(),
            select_stories: client.prepare(SQL_SELECT_STORIES).await.unwrap(),
            insert_story: client.prepare(SQL_INSERT_STORY).await.unwrap(),
            delete_story: client.prepare(SQL_DELETE_STORY).await.unwrap(),
        }
    }
}

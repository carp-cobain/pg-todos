use crate::Error;
use statements::Cache;
use tokio_postgres::{Client, Error as DbError, NoTls};

/// SQL queries
mod sql;
mod statements;
mod story;
mod task;

pub struct Repo {
    client: Client,
    statements: Cache,
}

impl Repo {
    pub async fn new(db_url: &str) -> Self {
        let (client, conn) = tokio_postgres::connect(db_url, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                panic!("connection error: {}", err);
            }
        });
        let statements = Cache::prepare(&client).await;
        Self { client, statements }
    }
}

impl From<DbError> for Error {
    fn from(err: DbError) -> Self {
        // fail fast because we don't re-connect.
        if err.is_closed() {
            panic!("db connection closed");
        }
        Error::internal(err.to_string())
    }
}

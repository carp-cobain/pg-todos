use crate::Error;
use statements::Statements;
use tokio_postgres::{Client, NoTls};

mod statements;
mod story;

pub struct Repo {
    client: Client,
    statements: Statements,
}

impl Repo {
    pub async fn new(db_url: &str) -> Self {
        tracing::debug!("new: connecting to {}", db_url);
        let (client, conn) = tokio_postgres::connect(db_url, NoTls)
            .await
            .expect("can not connect to postgresql");
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                tracing::error!("Connection error: {:?}", err);
            }
        });
        let statements = Statements::prepare(&client).await;
        Self { client, statements }
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        // Fail fast because we won't re-connect.
        if err.is_closed() {
            panic!("Connection closed: abort!");
        }
        Error::Internal {
            message: err.to_string(),
        }
    }
}

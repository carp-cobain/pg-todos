use crate::Error;
use statements::Statements;
use tokio_postgres::{Client, Error as DbError, NoTls};

mod statements;
mod story;

pub struct Repo {
    client: Client,
    statements: Statements,
}

impl Repo {
    pub async fn new(db_url: &str) -> Self {
        let (client, conn) = tokio_postgres::connect(db_url, NoTls)
            .await
            .expect("db connect failed");

        tokio::spawn(async move {
            if let Err(err) = conn.await {
                panic!("connection error: {:?}", err);
            }
        });

        let statements = Statements::prepare(&client).await;
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

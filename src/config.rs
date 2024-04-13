use percent_encoding::NON_ALPHANUMERIC;
use std::{env, io, net::SocketAddr};
use tokio::net::{TcpListener, TcpSocket};

/// Configuration settings
#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_database: String,
}

/// Default for config just calls basic constructor
impl Default for Config {
    fn default() -> Self {
        Self::load()
    }
}

impl Config {
    /// Load config from env vars.
    pub fn load() -> Self {
        // http server settings
        let port = env::var("HTTP_SERVER_PORT").unwrap_or("8080".into());
        let listen_addr = format!("0.0.0.0:{}", port);

        // database settings
        let db_host = env::var("DB_HOST").expect("DB_HOST not set");
        let db_port = env::var("DB_PORT")
            .unwrap_or("5432".to_owned())
            .parse()
            .expect("DB_PORT could not be parsed");
        let db_user = env::var("DB_USER").expect("DB_USER not set");
        let db_password = env::var("DB_PASS").expect("DB_PASS not set");
        let db_database = env::var("DB_NAME").expect("DB_NAME not set");

        // Create config
        Self {
            listen_addr,
            db_host,
            db_port,
            db_user,
            db_password,
            db_database,
        }
    }

    pub fn db_connection_string(&self) -> String {
        let bytes = self.db_password.as_bytes();
        let password = percent_encoding::percent_encode(bytes, NON_ALPHANUMERIC);
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, password, self.db_host, self.db_port, self.db_database,
        )
    }

    pub fn tcp_listener(&self) -> TcpListener {
        let addr: SocketAddr = self
            .listen_addr
            .parse()
            .expect("Failed to parse listen address");

        reuse_listener(addr).expect("Failed calling reuse_listener")
    }
}

fn reuse_listener(addr: SocketAddr) -> io::Result<TcpListener> {
    let socket = match addr {
        SocketAddr::V4(_) => TcpSocket::new_v4()?,
        SocketAddr::V6(_) => TcpSocket::new_v6()?,
    };
    #[cfg(unix)]
    {
        if let Err(e) = socket.set_reuseport(true) {
            tracing::warn!("error setting SO_REUSEPORT: {}", e);
        }
    }
    if let Err(e) = socket.set_reuseaddr(true) {
        tracing::warn!("error calling set_reuseaddr: {}", e);
    }
    if let Err(e) = socket.set_nodelay(true) {
        tracing::warn!("error calling set_nodelay: {}", e);
    }
    socket.bind(addr)?;
    socket.listen(1024)
}

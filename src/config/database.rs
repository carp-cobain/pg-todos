use crate::config::Config;
use percent_encoding::NON_ALPHANUMERIC;

impl Config {
    pub fn db_connection_string(&self) -> String {
        let bytes = self.db_password.as_bytes();
        let password = percent_encoding::percent_encode(bytes, NON_ALPHANUMERIC);
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, password, self.db_host, self.db_port, self.db_database,
        )
    }
}

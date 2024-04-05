pub mod api;
pub mod config;
pub mod domain;
pub mod error;
pub mod repo;

pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

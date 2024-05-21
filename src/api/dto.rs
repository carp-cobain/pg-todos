use crate::{Error, Result};
use serde::Deserialize;

/// The POST body for creating stories
#[derive(Debug, Deserialize)]
pub struct CreateStoryBody {
    pub name: String,
}

impl CreateStoryBody {
    /// Sanitize and validate story name from request body
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim();
        if name.is_empty() || name.len() > 100 {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name.to_string())
    }
}

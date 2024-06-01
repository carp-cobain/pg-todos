use crate::{Error, Result};
use serde::Deserialize;

/// The POST body for creating stories
#[derive(Debug, Deserialize)]
pub struct StoryBody {
    pub name: String,
}

impl StoryBody {
    /// Sanitize and validate story name from request body
    pub fn validate(&self) -> Result<String> {
        let name = self.name.trim();
        if name.is_empty() || name.len() > 100 {
            return Err(Error::invalid_args("name: invalid length"));
        }
        Ok(name.to_string())
    }
}

/// The POST body for creating tasks
#[derive(Debug, Deserialize)]
pub struct CreateTaskBody {
    pub name: String,
    pub story_id: i32,
}

impl CreateTaskBody {
    /// Sanitize and validate task name and story_id from request body
    pub fn validate(&self) -> Result<(i32, String)> {
        // Collects error messages
        let mut messages = Vec::new();

        // Validate body params
        let story_id = self.story_id;
        if story_id <= 0 {
            messages.push("story_id: must be > 0".into());
        }
        let name = self.name.trim();
        if name.is_empty() || name.len() > 100 {
            messages.push("name: invalid length".into());
        }

        // Return params or errors
        if messages.is_empty() {
            Ok((story_id, name.to_string()))
        } else {
            Err(Error::InvalidArgs { messages })
        }
    }
}

/// The PATCH body for updating tasks
#[derive(Debug, Deserialize)]
pub struct PatchTaskBody {
    pub name: Option<String>,
    pub status: Option<String>,
}

impl PatchTaskBody {
    /// Helper to validate fields to update for a task.
    pub fn validate(&self) -> Result<(Option<String>, Option<String>)> {
        // Make sure at least one field is provided
        if self.name.is_none() && self.status.is_none() {
            return Err(Error::invalid_args("name and/or status must be provided"));
        }

        // Defaults
        let mut name = None;
        let mut status = None;

        // Validate
        let mut messages = Vec::new();
        if let Some(n) = &self.name {
            let n = n.trim().to_string();

            if n.is_empty() || n.len() > 100 {
                messages.push("name: invalid length".into());
            } else {
                name = Some(n);
            }
        }

        if let Some(s) = &self.status {
            let stl = s.trim().to_lowercase();
            if ["complete".into(), "incomplete".into()].contains(&stl) {
                status = Some(stl);
            } else {
                messages.push("status: invalid enum variant".into());
            }
        }

        // Check for and return validation failures
        if !messages.is_empty() {
            return Err(Error::InvalidArgs { messages });
        }

        Ok((name, status))
    }
}

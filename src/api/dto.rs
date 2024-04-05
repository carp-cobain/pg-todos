use serde::Deserialize;
use std::fmt::Debug;
use validator::Validate;

// Min string length bytes
const MIN_LEN: u64 = 1;

// Max string length bytes
const MAX_LEN: u64 = 100;

/// The POST body for creating stories
#[derive(Debug, Default, Deserialize, Validate)]
pub struct CreateStoryBody {
    #[validate(length(min = "MIN_LEN", max = "MAX_LEN", message = "invalid length"))]
    pub name: String,
}

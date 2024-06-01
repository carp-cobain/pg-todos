use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Story {
    pub id: i32,
    pub name: String,
}

impl Story {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Task {
    pub id: i32,
    pub story_id: i32,
    pub name: String,
    pub status: String,
}

impl Task {
    pub fn new(id: i32, story_id: i32, name: String, status: String) -> Self {
        Self {
            id,
            story_id,
            name,
            status,
        }
    }
}

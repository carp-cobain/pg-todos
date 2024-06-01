/// SQL queries for the stories table
pub mod stories {
    pub const FETCH: &str = "select id, name from stories where id = $1";
    pub const SELECT: &str =
        "select id, name from stories where id <= $1 order by id desc limit 100";
    pub const INSERT: &str = "insert into stories (name) values ($1) returning id";
    pub const UPDATE: &str = "update stories set name = $1 where id = $2";
    pub const DELETE: &str = "delete from stories where id = $1";
}

/// SQL queries for the tasks table
pub mod tasks {
    pub const FETCH: &str = "select id, story_id, name, status from tasks where id = $1";
    pub const SELECT: &str =
        "select id, story_id, name, status from tasks where story_id = $1 order by id limit 100";
    pub const INSERT: &str =
        "insert into tasks (story_id, name) values ($1, $2) returning id, status";
    pub const UPDATE: &str =
        "update tasks set name = $1, status = $2 where id = $3 returning story_id";
    pub const DELETE: &str = "delete from tasks where id = $1";
}

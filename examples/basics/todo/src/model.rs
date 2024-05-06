use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct NewTask {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub description: String,
    pub completed: bool,
}

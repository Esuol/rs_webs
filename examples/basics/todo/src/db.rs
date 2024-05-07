use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::model::{NewTask, Task};

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(database_url)
        .await
}

pub async fn get_all_tasks(pool: &SqlitePool) -> Result<Vec<Task>, &'static str> {
    Task::all(pool).await.map_err(|_| "Error retrieving tasks")
}

pub async fn create_task(todo: String, pool: &SqlitePool) -> Result<(), &'static str> {
    let new_task = NewTask { description: todo };
}

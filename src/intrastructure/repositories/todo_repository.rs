use surrealdb::SurrealDb;
use surrealdb::sql::Thing;
use async_std::task;
use anyhow::{Result, Error};
use chrono::Utc;
use std::sync::Mutex;

// Define your Todo struct here (assuming it's defined elsewhere)
// For example:
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Todo {
//     pub id: Option<Thing>,
//     pub content: String,
//     pub completed: bool,
//     pub title: String,
//     pub createdAt: DateTime<Utc>,
//     pub updatedAt: DateTime<Utc>,
// }


pub struct TodoRepository {
    db: std::sync::Arc<Mutex<SurrealDb>>,
}

impl TodoRepository {
    pub fn new(db: std::sync::Arc<Mutex<SurrealDb>>) -> Self {
        Self { db }
    }

    pub async fn create_todo(&self, content: String, title: String) -> Result<Vec<Todo>, Error> {
        let mut db = self.db.lock().await;
        let result = db
            .create("todo")
            .content(Todo {
                id: None, // SurrealDB will generate an ID
                content,
                completed: false,
                title,
                createdAt: Utc::now(),
                updatedAt: Utc::now(),
            })
            .await?;

        // Handle the case where `db.create` might return a single Todo object
        let created_todos: Vec<Todo> = match result {
            Some(todos) => todos, // Assuming it's already a Vec<Todo> if successful and returning Some(Vec<Todo>)
            None => {
                // This case might happen if create returns None on error or in unexpected situations.
                // You might want to handle errors more explicitly based on SurrealDB's behavior.
                vec![] // Return an empty vector or handle the error appropriately
            }
        };
        Ok(created_todos)
    }

    // ... other functions in your repository ...
}

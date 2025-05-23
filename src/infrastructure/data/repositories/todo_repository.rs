use crate::domain::models::todo::Todo;
use crate::infrastructure::data::db_context::surreal_context::DB;
use surrealdb::{Error, error::Db::Thrown};

#[derive(Debug)]
pub struct TodoRepository {
    table: String,
}
impl Default for TodoRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoRepository {
    pub fn new() -> Self {
        TodoRepository {
            table: String::from("todo"),
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let records = DB.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Todo, Error> {
        if let Some(record) = DB.select((&self.table, id.clone())).await? {
            return Ok(record);
        }

        let error = Error::Db(Thrown(format!("Todo with id {} not found", id)));
        Err(error)
    }

    pub async fn get_by_title(&self, title: String) -> Result<Todo, Error> {
        if let Some(record) = DB
            .query("SELECT * FROM todo WHERE title = $title")
            .bind(("title", title.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        let error = Error::Db(Thrown(format!("Todo with title {} not found", title)));
        Err(error)
    }

    #[tracing::instrument(ret)]
    pub async fn create_todo(&self, content: Todo) -> Result<Todo, Error> {
        if let Some(record) = DB.create(&self.table).content(content).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown("Todo could not be created".to_string()));
        Err(error)
    }

    pub async fn update_todo(&self, id: String, content: Todo) -> Result<Todo, Error> {
        let record = DB
            .update((&self.table, id))
            .content(content)
            .await?
            .unwrap();
        Ok(record)
    }

    pub async fn delete_todo(&self, id: String) -> Result<Todo, Error> {
        let result = DB.delete((&self.table, id)).await?.unwrap();
        Ok(result)
    }
}

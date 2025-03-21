use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Local>>,
    pub updatedAt: Option<DateTime<Local>>,
}

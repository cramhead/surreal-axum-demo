

use axum::{Json, http::StatusCode, response::IntoResponse};
use chrono::Local;
use tracing::info;

use crate::{
    domain::models::todo::Todo, infrastructure::data::repositories::todo_repository::TodoRepository,
};

#[tracing::instrument(ret)]
pub async fn create_todo_command(
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let repository = TodoRepository::new();

    if let Ok(todo) = repository.get_by_title(body.title.clone()).await {
        let json_response = serde_json::json!({
            "status": "error",
            "message": "Todo already exists",
            "data": todo,
        });

        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    } else {
        let datetime = Local::now();
        body.completed = Some(false);
        body.createdAt = Some(datetime);
        body.updatedAt = Some(datetime);

        let todo = body.to_owned();

        match repository.create_todo(todo.clone()).await {
            Ok(todo) => {
                
                info!("Todo created");
                info!("The database created todo: {:?}", todo);
                let json_response = serde_json::json!({
                    "status": "success",
                    "data": todo,
                });

                Ok((StatusCode::CREATED, Json(json_response)))
            }
            Err(e) => {
                let json_response = serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to create todo. Error: {}", e.to_string())
                });

                Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json_response)))
            }
        }
    }
}

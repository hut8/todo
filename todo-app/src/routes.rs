use crate::DbConn;
use rocket::serde::{json::Json};
use crate::models::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
pub async fn tasks_index(conn: DbConn) -> Json<Vec<Task>> {
    let tasks: Vec<Task> = conn
    .run(|c| load_tasks(c))
    .await;
    Json(tasks)
}

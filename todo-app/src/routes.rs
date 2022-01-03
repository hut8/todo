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
    .run(|c| Task::all(c))
    .await;
    Json(tasks)
}

#[post("/", format="application/json", data = "<new_task>")]
pub async fn tasks_create(conn: DbConn, new_task: Json<TaskForm>) -> Json<Task> {
    let t = new_task.0;
    let task = conn.run(|c| Task::create(c, t)).await;
    Json(task)
}
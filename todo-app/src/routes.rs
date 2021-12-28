use crate::DbConn;
use chrono::Utc;
use rocket::serde::{json::Json};
use crate::models::*;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tasks")]
pub fn tasks_index(mut conn: DbConn) -> Json<Task> {
    Json(Task{
        completed: false,
        created_at: Utc::now(),
        description: "test".into(),
    })
}

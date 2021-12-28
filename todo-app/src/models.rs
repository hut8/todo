use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use chrono::{DateTime, Utc};

#[derive(Serialize,Deserialize)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

pub fn new_task(description: String) -> Task {
    Task{
        description,
        completed: false,
        created_at: Utc::now(),
    }
}
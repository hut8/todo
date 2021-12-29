use rocket::serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::NaiveDateTime,
}

pub fn load_tasks(conn: &diesel::PgConnection) -> Vec<Task> {
    let results = tasks
    .filter(completed.eq(false))
    .load::<Task>(conn).expect("load tasks");
    results
}

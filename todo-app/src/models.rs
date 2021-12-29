use rocket::serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use crate::schema::tasks;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

impl Task {
    pub fn all(conn: &diesel::PgConnection) -> Vec<Task> {
        use crate::schema::tasks::dsl::*;
        let results = tasks
        .filter(completed.eq(false))
        .load::<Task>(conn).expect("load tasks");
        results
    }
    
    pub fn create(conn: &diesel::PgConnection, t: NewTask) -> Task {
        use crate::schema::tasks::dsl::*;
        let result = diesel::insert_into(tasks)
        .values(t)
        .get_result(conn).expect("create task");
        result
    }
}


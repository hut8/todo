use rocket::serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use crate::schema::tasks;

#[derive(Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct TaskForm {
    pub description: String,
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
    
    pub fn create(conn: &diesel::PgConnection, t: TaskForm) -> Task {
        use crate::schema::tasks::dsl::*;
        let task = NewTask{
            description: t.description,
            completed: false,
            created_at: Utc::now().naive_utc(),
        };
        let result = diesel::insert_into(tasks)
        .values(task)
        .get_result(conn).expect("create task");
        result
    }

    pub fn get(conn: &diesel::PgConnection, id: i32) -> Task {
        use crate::schema::tasks::dsl::tasks;
        let result = tasks.find(id).first::<Task>(conn).expect("get task");
        result
    }

    pub fn delete(conn: &diesel::PgConnection, id: i32) {
        use crate::schema::tasks::dsl::tasks;
        diesel::delete(tasks.find(id)).execute(conn).expect("delete task");
    }
}


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

#[cfg(test)]
mod test {
    use crate::models::TaskForm;
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn create_task() {
        let form = TaskForm{
            description: "New Task".into(),
        };
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/tasks").json(&form).dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}
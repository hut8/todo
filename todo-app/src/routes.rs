use crate::DbConn;
use rocket::{serde::{json::Json}, response::status};
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
pub async fn tasks_create(conn: DbConn, new_task: Json<TaskForm>) -> status::Created<Json<Task>> {
    let t = new_task.0;
    let task = conn.run(|c| Task::create(c, t)).await;
    let loc = uri!(tasks_show(id=task.id)); // format!("/tasks/{}", task.id)
    status::Created::new(loc.to_string())
        .body(Json(task))
}

#[get("/<id>")]
pub async fn tasks_show(conn: DbConn, id: i32) -> Json<Task> {
    let task = conn.run(move |c| Task::get(c, id)).await;
    Json(task)
}

#[delete("/<id>")]
pub async fn tasks_delete(conn: DbConn, id: i32) -> status::NoContent {
    conn.run(move |c| Task::delete(c, id)).await;
    status::NoContent
}


#[cfg(test)]
mod test {
    use crate::models::TaskForm;
    use crate::rocket;
    use rocket::local::asynchronous;
    use rocket::local::blocking::Client;
    use rocket::http::{ContentType, Status};

    #[test]
    fn tasks_create() {
        let form = TaskForm{
            description: "New Task".into(),
        };
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/tasks").json(&form).dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }

    #[async_test]
    async fn tasks_show() {
        let r = rocket();
        let client = asynchronous::Client::tracked(r).await.expect("valid rocket instance");
        let db = crate::DbConn::get_one(client.rocket()).await.unwrap();
        let t = db.run(|c| crate::models::Task::create(c , TaskForm{
            description: String::from("test task"),
        })).await;
        // let u = uri!(crate::tasks_show(id=t.id));
        let response = client.get(format!("/tasks/{}", t.id)).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
    }
}
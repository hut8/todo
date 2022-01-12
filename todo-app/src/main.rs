pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;

extern crate chrono;
extern crate serde;
extern crate serde_json;
// #[macro_use]
// extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[rocket_sync_db_pools::database("todo")]
pub struct DbConn(diesel::PgConnection);


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(DbConn::fairing())
    .attach(cors::CorsFairing)
    .mount("/", routes![routes::index])
    .mount("/tasks", routes![
        routes::tasks_index,
        routes::tasks_create])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
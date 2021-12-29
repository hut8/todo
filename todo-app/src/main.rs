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
    println!("TODO App");
    rocket::build()
    .attach(DbConn::fairing())
    .attach(cors::CorsFairing)
    .mount("/", routes![routes::index])
    .mount("/tasks", routes![routes::tasks_index])
}
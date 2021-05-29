#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod db;

// #[post("/threads")]
// async fn create_threads() -> &'static str {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[get("/async")]
async fn async_hello() -> &'static str {
    rocket::tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    "Hello, Rocket! I send this message 1s later"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, async_hello])
}

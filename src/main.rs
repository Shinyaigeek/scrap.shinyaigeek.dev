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

use crate::db::_repositories::threads::create::{create, NewThread};
use crate::db::_repositories::threads::read::reads;
use crate::db::connection::establish::establish_connection;

mod db;

// #[post("/threads")]
// async fn create_threads() -> &'static str {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

#[get("/post_threads")]
fn pos() -> &'static str {
    let connection = establish_connection();
    create(
        NewThread {
            title: "asdf".to_string(),
            published: true,
        },
        connection,
    );
    "dekita"
}

#[get("/threads")]
fn asdf() -> String {
    let connection = establish_connection();
    let threads = reads(connection);
    format!("title: {:?}", threads[0].title)
}

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
    rocket::build().mount("/", routes![index, async_hello, pos, asdf])
}

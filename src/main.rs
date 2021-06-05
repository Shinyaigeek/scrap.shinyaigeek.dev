#![feature(proc_macro_hygiene, decl_macro)]

use actix_web::{get, web, App, HttpServer, Responder};

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

// async fn create_threads() -> impl Responder {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

async fn pos() -> impl Responder {
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

async fn asdf() -> impl Responder {
    let connection = establish_connection();
    let threads = reads(connection);
    format!("title: {:?}", threads[0].title)
}

async fn index() -> impl Responder {
    "Hello, Rocket!"
}

async fn async_hello() -> impl Responder {
    "Hello, Rocket! I send this message 1s later"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/async", web::get().to(async_hello))
            .route("/threads", web::get().to(asdf))
            .route("/post_threads", web::get().to(pos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

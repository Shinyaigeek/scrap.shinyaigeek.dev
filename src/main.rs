#![feature(proc_macro_hygiene, decl_macro)]

use actix_web::{get, web, App, HttpServer, Responder, HttpRequest};

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
use crate::routes::users::signin;

mod db;
mod routes;

fn get_auth_from_header(req: HttpRequest) -> Option<String> {
    let auth = req.headers().get("Authorization");
    let auth = match auth {
        Some(a) => Some(a.to_str().unwrap().replace("Bearer ", "")),
        None => None
    };

    auth
}

async fn dispatch_signin(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req.headers().get("accept"));
    "asdf"
}

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
            .route("/signin", web::get().to(dispatch_signin))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn get_auth() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        assert_eq!(get_auth_from_header(req), None);
        let req = test::TestRequest::with_header("Authorization", "Bearer asdf").to_http_request();
        assert_eq!(get_auth_from_header(req), Some("asdf".to_string()));
    }
}
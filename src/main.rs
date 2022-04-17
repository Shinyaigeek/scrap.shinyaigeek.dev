#![feature(proc_macro_hygiene, decl_macro)]

use actix_web::{
    dev, get, http, post, web, App, HttpRequest as ActixHttpRequest,
    HttpResponse as ActixHttpResponse, HttpServer, Responder,
};

use actix_web::http::Method as ActixMethod;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::auth::get_gh_info_with_token::get_gh_info_with_token;
use crate::db::applications::users::signin::signin;
use crate::db::applications::users::signup::signup;
use crate::db::connection::establish::establish_connection;
use crate::routes::threads::create::threads_create;
use crate::routes::threads::read::{thread_read, threads_read};
use crate::routes::users::signin;
use crate::routes::{HttpRequest, HttpRequestMethod, HttpResponse, GET};
use crate::util::http_client::HttpClient;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
mod auth;
mod db;
mod routes;
mod util;

#[derive(Serialize, Deserialize)]
struct ErrMessage {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct UserMessage {
    gh_id: String,
}

#[derive(Serialize, Deserialize)]
struct ThreadSlug {
    slug: String,
}

#[derive(Serialize, Deserialize)]
struct Thread {
    title: String,
    slug: String,
    content: String,
    published: bool,
}

fn get_auth_from_header(req: ActixHttpRequest) -> Option<String> {
    let auth = req.headers().get("Authorization");
    let auth = match auth {
        Some(a) => Some(a.to_str().unwrap().replace("Bearer ", "")),
        None => None,
    };

    auth
}

fn actix_request_into_http_request(req: ActixHttpRequest) -> HttpRequest {
    HttpRequest {
        path: req.path().to_string(),
        method: match req.method() {
            &ActixMethod::GET => HttpRequestMethod::GET,
            &ActixMethod::POST => HttpRequestMethod::POST,
            // TODO
            _ => panic!("todo task; HTTP Request Method"),
        },
        authorization: match req.head().headers.get("Authorization") {
            Some(a) => Some(a.to_str().unwrap().replace("Bearer ", "")),
            None => None,
        },
    }
}

fn http_response_into_actix_response(res: HttpResponse) -> ActixHttpResponse {
    let mut base_response = match res.status {
        200 => ActixHttpResponse::Ok(),
        500 => ActixHttpResponse::InternalServerError(),
        // TODO
        _ => ActixHttpResponse::InternalServerError(),
    };

    base_response
        .content_type("application/json")
        .body(res.body)
}

async fn dispatch_signin(req: ActixHttpRequest) -> impl Responder {
    let connection = establish_connection();
    let idToken = get_auth_from_header(req);
    let idToken = match idToken {
        Some(token) => token,
        None => {
            return ActixHttpResponse::BadRequest().json(ErrMessage {
                message: "Authorization header field must be exist with /signin request"
                    .to_string(),
            });
        }
    };
    let client = HttpClient::new();
    let gh_username = get_gh_info_with_token(idToken, client).await;

    let user = signin(gh_username, connection);

    let user = match user {
        Some(i) => i,
        None => {
            return ActixHttpResponse::BadRequest().json(ErrMessage {
                message: "There is no user, please signup before signin".to_string(),
            });
        }
    };

    ActixHttpResponse::Ok().json(UserMessage {
        gh_id: user.gh_user_id,
    })
}

async fn dispatch_signup(req: ActixHttpRequest) -> impl Responder {
    let idToken = get_auth_from_header(req);
    let idToken = match idToken {
        Some(token) => token,
        None => {
            return ActixHttpResponse::BadRequest().json(ErrMessage {
                message: "Authorization header field must be exist with /signup request"
                    .to_string(),
            });
        }
    };
    let client = HttpClient::new();
    let gh_username = get_gh_info_with_token(idToken, client).await;

    let connection = establish_connection();

    let user = signup(gh_username, connection);

    match user {
        Ok(user) => ActixHttpResponse::Ok().json(UserMessage {
            gh_id: user.gh_user_id,
        }),
        Err(_) => {
            return ActixHttpResponse::InternalServerError().json(ErrMessage {
                message: "oops!! signup was failed.".to_string(),
            });
        }
    }
}

async fn dispatch_threads_create(
    req: ActixHttpRequest,
    payload: web::Json<Thread>,
) -> impl Responder {
    let response = threads_create(
        payload.title.to_string(),
        payload.slug.to_string(),
        payload.content.to_string(),
        payload.published,
    );
    http_response_into_actix_response(response)
}

async fn dispatch_thread_read(
    req: ActixHttpRequest,
    body: web::Path<ThreadSlug>,
) -> impl Responder {
    let request = actix_request_into_http_request(req);
    let response = thread_read(request, body.slug.to_string());
    http_response_into_actix_response(response)
}

async fn dispatch_threads_read(req: ActixHttpRequest) -> impl Responder {
    let request = actix_request_into_http_request(req);
    let response = threads_read(request);
    http_response_into_actix_response(response)
}

async fn index() -> impl Responder {
    "Hello, Rocket!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .route("/", web::get().to(index))
            .route("/signin", web::get().to(dispatch_signin))
            .route("/signup", web::post().to(dispatch_signup))
            .route("/threads", web::get().to(dispatch_threads_read))
            .route("/threads/create", web::post().to(dispatch_threads_create))
            .route("/threads/{slug}", web::get().to(dispatch_thread_read))
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

#![feature(proc_macro_hygiene, decl_macro)]

use actix_web::{
    get, http, web, App, HttpRequest as ActixHttpRequest, HttpResponse as ActixHttpResponse,
    HttpServer, Responder,
};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::auth::get_gh_info_with_token::get_gh_info_with_token;
use crate::db::applications::threads::create::create_thread;
use crate::db::applications::users::signin::signin;
use crate::db::applications::users::signup::signup;
use crate::db::connection::establish::establish_connection;
use crate::routes::threads::read::threads_read;
use crate::routes::users::signin;
use crate::routes::{HttpRequest, HttpRequestMethod, HttpResponse};
use crate::util::http_client::HttpClient;
use actix_cors::Cors;
use serde::Serialize;
mod auth;
mod db;
mod routes;
mod util;

#[derive(Serialize)]
struct ErrMessage {
    message: String,
}

#[derive(Serialize)]
struct UserMessage {
    gh_id: String,
}

fn get_auth_from_header(req: ActixHttpRequest) -> Option<String> {
    let auth = req.headers().get("Authorization");
    let auth = match auth {
        Some(a) => Some(a.to_str().unwrap().replace("Bearer ", "")),
        None => None,
    };

    auth
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

async fn pos() -> impl Responder {
    let connection = establish_connection();
    create_thread(
        "asdf".to_string(),
        "nya-n".to_string(),
        "hogehoge".to_string(),
        true,
        connection,
    );
    "dekita"
}

async fn dispatch_threads_read() -> impl Responder {
    let request = HttpRequest {
        path: "/threads/read".to_string(),
        method: HttpRequestMethod::GET,
    };
    let response = threads_read(request);
    response.body
}

async fn index() -> impl Responder {
    "Hello, Rocket!"
}

async fn async_hello() -> impl Responder {
    let client = HttpClient::new();
    let name = get_gh_info_with_token("eyJhbGciOiJSUzI1NiIsImtpZCI6ImFiMGNiMTk5Zjg3MGYyOGUyOTg5YWI0ODFjYzJlNDdlMGUyY2MxOWQiLCJ0eXAiOiJKV1QifQ.eyJuYW1lIjoiU2hpbm9idSBIYXlhc2hpIiwicGljdHVyZSI6Imh0dHBzOi8vYXZhdGFycy5naXRodWJ1c2VyY29udGVudC5jb20vdS80Mjc0MjA1Mz92PTQiLCJpc3MiOiJodHRwczovL3NlY3VyZXRva2VuLmdvb2dsZS5jb20vc2NyYXAtc2hpbnlhaWdlZWstZGV2IiwiYXVkIjoic2NyYXAtc2hpbnlhaWdlZWstZGV2IiwiYXV0aF90aW1lIjoxNjIyOTcwMzMwLCJ1c2VyX2lkIjoibnN5bkt1Z1c5RVNwUzZrWXR0UmVpNVVoVm1IMyIsInN1YiI6Im5zeW5LdWdXOUVTcFM2a1l0dFJlaTVVaFZtSDMiLCJpYXQiOjE2MjI5NzAzMzAsImV4cCI6MTYyMjk3MzkzMCwiZW1haWwiOiJtZUBzaGlueWFpZ2Vlay5kZXYiLCJlbWFpbF92ZXJpZmllZCI6ZmFsc2UsImZpcmViYXNlIjp7ImlkZW50aXRpZXMiOnsiZ2l0aHViLmNvbSI6WyI0Mjc0MjA1MyJdLCJlbWFpbCI6WyJtZUBzaGlueWFpZ2Vlay5kZXYiXX0sInNpZ25faW5fcHJvdmlkZXIiOiJnaXRodWIuY29tIn19.fyQixQAUamGfmzn4Z-2h6AliixEwPEbVnTcs1CWKDKcMj9AnYYkNHCRdQb3v39aZQAtoJIakrvD7qIBeBmZmydLmbsB38I2GxiNJ6EHEsm_kXGpcpjLblKyQWTf5njvvrxff8hRdB_ODHFw-mnlELIagLNboD00aPT-i3kTzRBMjKKrbmSKJM47XSxtDKGcnitx-jOrWUA-XTVbrvfCMaNF7I4JD6HssiGn-vxUp9D09SpL9L6JVCCsaTTc-TCQBl94T19qBXonzW2UMhzZSKFtFyHgR3DNZWsiKF1mRgN6xIhwTuOT7WOUhUBnrW7jsYxVrhLGYhp7MsiP7GAuWMA".to_string(), client).await;
    format!("Hello, Rocket! I send this message 1s {}", name)
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
            .route("/async", web::get().to(async_hello))
            .route("/post_threads", web::get().to(pos))
            .route("/signin", web::get().to(dispatch_signin))
            .route("/signup", web::post().to(dispatch_signup))
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

use crate::db::applications::comments::read::read_comments;
use crate::db::connection::establish::establish_connection;
use crate::routes::{HttpRequest, HttpResponse};

use super::super::util::make_error_response_body::make_error_response_body;

// TODO
pub fn comments_read(req: HttpRequest, thread_id: i32) -> HttpResponse {
    let connection = establish_connection();
    let comments = read_comments(thread_id, connection);
    match comments {
        Ok(comments) => {
            return HttpResponse {
                status: 200,
                body: { serde_json::to_string(&comments).unwrap_or("[]".to_string()) },
            }
        }

        Err(_) => {
            return HttpResponse {
                status: 500,
                body: make_error_response_body(500),
            }
        }
    }
}

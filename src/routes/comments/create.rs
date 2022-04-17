use crate::db::applications::comments::create::create_comment;
use crate::db::connection::establish::establish_connection;
use crate::routes::{HttpRequest, HttpResponse};

use super::super::util::make_error_response_body::make_error_response_body;

pub fn comment_create(content: String, author: String, thread: i32) -> HttpResponse {
    let connection = establish_connection();
    let comment = create_comment(content, author, thread, connection);
    match comment {
        Ok(comment) => {
            return HttpResponse {
                status: 200,
                body: {
                    let mut body = String::new();
                    body.push_str(&format!(
                        "{{
                                \"content\": {:?},
                                \"author\": {:?},
                                \"thread\": {:?}
                            }}",
                        comment.content, comment.author, comment.thread
                    ));

                    body
                },
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

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
                body: {
                    if comments.len() == 0 {
                        "[]".to_string()
                    } else {
                        let mut body = "[".to_string();

                        let mut idx = 0;

                        let comments_len = comments.len();

                        for comment in comments {
                            body.push_str(&format!(
                                "{{
                                \"content\": {:?},
                                \"author\": {:?},
                                \"thread\": {:?}
                            }}",
                                comment.content, comment.author, comment.thread
                            ));

                            if idx < comments_len - 1 {
                                body.push(',');
                                idx += 1;
                            }
                        }

                        body.push_str("]");
                        body
                    }
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

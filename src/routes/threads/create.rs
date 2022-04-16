use crate::db::applications::threads::read::read_threads;
use crate::db::connection::establish::establish_connection;
use crate::routes::{HttpRequest, HttpResponse};

use super::super::util::make_error_response_body::make_error_response_body;

pub fn threads_create(req: HttpRequest) -> HttpResponse {
    let connection = establish_connection();
    let threads = read_threads(-1, connection);
    match threads {
        Ok(threads) => {
            return HttpResponse {
                status: 200,
                body: {
                    if threads.len() == 0 {
                        "[]".to_string()
                    } else {
                        let mut body = "[".to_string();

                        let mut idx = 0;

                        let threads_len = threads.len();

                        for thread in threads {
                            body.push_str(&format!(
                                "{{
                                \"title\": {:?},
                                \"slug\": {:?},
                                \"content\": {:?},
                                \"is_open\": {:?}
                            }}",
                                thread.title, thread.slug, thread.content, thread.is_open
                            ));

                            if idx < threads_len - 1 {
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

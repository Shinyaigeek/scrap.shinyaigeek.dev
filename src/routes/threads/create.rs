use crate::db::applications::threads::create::create_thread;
use crate::db::connection::establish::establish_connection;
use crate::routes::{HttpRequest, HttpResponse};

use super::super::util::make_error_response_body::make_error_response_body;

pub fn threads_create(
    title: String,
    slug: String,
    content: String,
    published: bool,
) -> HttpResponse {
    let connection = establish_connection();
    let thread = create_thread(title, slug, content, published, connection);
    match thread {
        Ok(thread) => {
            return HttpResponse {
                status: 200,
                body: {
                    let mut body = String::new();
                    body.push_str(&format!(
                        "{{
                                \"title\": {:?},
                                \"slug\": {:?},
                                \"content\": {:?},
                                \"is_open\": {:?},
                                \"id\": {:?}
                            }}",
                        thread.title, thread.slug, thread.content, thread.is_open, thread.id
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

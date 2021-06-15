pub mod threads;
pub mod users;
mod util;

pub const GET: &str = "GET";

pub enum HttpRequestMethod {
    GET,
}

pub struct HttpRequest {
    pub path: String,
    pub method: HttpRequestMethod,
}
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

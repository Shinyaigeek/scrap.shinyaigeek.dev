pub mod threads;
pub mod users;

pub const GET: &str = "GET";

pub enum HttpRequestMethod {
    GET,
}

pub struct HttpRequest {
    pub path: String,
    pub method: HttpRequestMethod,
}
pub struct HttpResponse {
    pub status: u8,
    pub body: String,
}

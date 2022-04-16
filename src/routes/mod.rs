pub mod threads;
pub mod users;
mod util;

pub const GET: &str = "GET";
pub const POST: &str = "POST";

pub enum HttpRequestMethod {
    GET,
    POST
}

pub struct HttpRequest {
    pub path: String,
    pub method: HttpRequestMethod,
    pub authorization: Option<String>,
}
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

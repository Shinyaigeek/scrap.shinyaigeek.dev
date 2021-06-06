use hyper::{Client, Body, Uri};
use hyper::client::{HttpConnector};
use hyper::body::to_bytes;

pub struct HttpClient {
    client: Client<HttpConnector, Body>
}

pub struct HttpResponse {
    pub body: String
}

impl HttpClient {
    pub fn new() -> Self {
        return Self {
            client: Client::new()
        }
    }

    pub async fn get(self, url: &str) -> HttpResponse {
        let uri = url.parse::<Uri>().unwrap();
        let res = self.client.get(uri).await;
        let mut res = res.unwrap().into_body();
        let body = to_bytes(res).await;
        let body = body.unwrap();
        let body = String::from_utf8(body.to_vec()).unwrap();
        HttpResponse {
            body
        }
    }
}